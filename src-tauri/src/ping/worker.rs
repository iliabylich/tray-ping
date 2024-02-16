use std::{
    collections::VecDeque,
    sync::{Mutex, OnceLock},
};

use crate::{
    fixed_size_deque::FixedSizeDeque,
    ping::{
        dns_lookup::{hostname_to_ip_addr, DnsError},
        PingResult,
    },
};

const QUEUE_SIZE: usize = 15;

pub(crate) struct Worker {
    hostname_and_ip_addr: Option<(String, std::net::IpAddr)>,
    icmp_seq: u64,
    queue: FixedSizeDeque<QUEUE_SIZE, PingResult>,
    on_tick: Option<Box<dyn Fn(&VecDeque<Option<PingResult>>) + Send>>,
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Worker")
            .field("hostname_and_ip_addr", &self.hostname_and_ip_addr)
            .field("icmp_seq", &self.icmp_seq)
            .finish()
    }
}

static INSTANCE: OnceLock<Mutex<Worker>> = OnceLock::new();

fn worker() -> &'static Mutex<Worker> {
    INSTANCE
        .get()
        .expect("Worker not initialized, Worker::init() must be called first")
}

impl Worker {
    fn new() -> Self {
        Self {
            hostname_and_ip_addr: None,
            icmp_seq: 0,
            queue: FixedSizeDeque::new(),
            on_tick: None,
        }
    }

    pub(crate) fn init() {
        INSTANCE
            .set(Mutex::new(Self::new()))
            .expect("Worker already initialized");

        std::thread::spawn(|| loop {
            {
                worker().lock().expect("Worker lock poisoned").tick();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
    }

    pub(crate) fn set_hostname(hostname: &str) -> Result<(), DnsError> {
        let mut worker = worker().lock().expect("Worker lock poisoned");

        match hostname_to_ip_addr(hostname) {
            Ok(ip_addr) => {
                worker.hostname_and_ip_addr = Some((hostname.to_string(), ip_addr));
                Ok(())
            }
            Err(e) => {
                worker.hostname_and_ip_addr = None;
                Err(e)
            }
        }
    }

    fn tick(&mut self) {
        let result = if let Some((hostname, ip_addr)) = self.hostname_and_ip_addr.as_ref() {
            self.icmp_seq += 1;
            Some(ping(hostname, *ip_addr, self.icmp_seq))
        } else {
            None
        };

        self.queue.push(result);

        if let Some(f) = self.on_tick.as_ref() {
            f(self.queue.get());
        }
    }

    pub(crate) fn subscribe<F>(f: F)
    where
        F: Fn(&VecDeque<Option<PingResult>>) + Send + 'static,
    {
        let mut worker = worker().lock().expect("Worker lock poisoned");
        worker.on_tick = Some(Box::new(f));
    }
}

pub(crate) fn ping(hostname: &str, ip_addr: std::net::IpAddr, icmp_seq: u64) -> PingResult {
    let ttl = 64;
    let start = std::time::Instant::now();

    match ping::dgramsock::ping(ip_addr, None, Some(ttl), None, None, None) {
        Ok(_) => {}
        Err(e) => return PingResult::Error(e.to_string()),
    }

    let duration = start.elapsed();

    PingResult::Done {
        hostname: hostname.to_string(),
        icmp_seq,
        ttl,
        duration,
    }
}

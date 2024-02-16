use std::{
    collections::VecDeque,
    sync::{Mutex, OnceLock},
};

use crate::ping::{
    dns_lookup::{hostname_to_ip_addr, DnsError},
    PingResult,
};

#[derive(Debug)]
pub(crate) struct Worker {
    hostname_and_ip_addr: Option<(String, std::net::IpAddr)>,
    icmp_seq: u64,
    results_to_keep: usize,
    last_results: VecDeque<PingResult>,
}

static INSTANCE: OnceLock<Mutex<Worker>> = OnceLock::new();

fn worker() -> &'static Mutex<Worker> {
    INSTANCE
        .get()
        .expect("Worker not initialized, Worker::init() must be called first")
}

impl Worker {
    fn new(results_to_keep: usize) -> Self {
        let mut last_results = VecDeque::new();
        for _ in 0..results_to_keep {
            last_results.push_back(PingResult::NotConfigured);
        }

        Self {
            hostname_and_ip_addr: None,
            icmp_seq: 0,
            results_to_keep,
            last_results,
        }
    }

    pub(crate) fn init(results_to_keep: usize) {
        INSTANCE
            .set(Mutex::new(Self::new(results_to_keep)))
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

    pub(crate) fn current_stats() -> VecDeque<PingResult> {
        worker()
            .lock()
            .expect("Worker lock poisoned")
            .last_results
            .clone()
    }

    fn tick(&mut self) {
        let result = if let Some((hostname, ip_addr)) = self.hostname_and_ip_addr.as_ref() {
            self.icmp_seq += 1;
            ping(hostname, *ip_addr, self.icmp_seq)
        } else {
            PingResult::NotConfigured
        };

        self.last_results.push_back(result);
        if self.last_results.len() > self.results_to_keep {
            self.last_results.pop_front();
        }
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
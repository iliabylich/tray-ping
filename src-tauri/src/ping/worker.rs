use std::sync::mpsc::{Receiver, Sender};

use crate::{
    fixed_size_deque::FixedSizeDeque,
    ping::{
        dns_lookup::{hostname_to_ip_addr, DnsError},
        PingResult,
    },
};

use super::PingLine;

pub(crate) struct Worker<const N: usize> {
    hostname: String,
    ip_addr: std::net::IpAddr,
    icmp_seq: u64,
    queue: FixedSizeDeque<N, PingResult>,
    send_pings: Sender<[Option<PingResult>; N]>,
}

impl<const N: usize> Worker<N> {
    pub(crate) fn init(hostname: &str) -> Result<Receiver<[Option<PingResult>; N]>, DnsError> {
        let (send_pings, recv_pings) = std::sync::mpsc::channel();

        let hostname = hostname.to_string();

        let ip_addr = hostname_to_ip_addr(&hostname)?;

        let mut worker = Worker {
            hostname,
            ip_addr,
            icmp_seq: 0,
            queue: FixedSizeDeque::new(),
            send_pings,
        };

        std::thread::spawn(move || loop {
            worker.tick();
            std::thread::sleep(std::time::Duration::from_secs(1));
        });

        Ok(recv_pings)
    }

    fn tick(&mut self) {
        self.icmp_seq += 1;
        self.queue.push(PingLine::new(
            self.hostname.clone(),
            self.ip_addr,
            self.icmp_seq,
        ));

        self.send_pings
            .send(self.queue.get())
            .expect("Failed to send current ping status");
    }
}

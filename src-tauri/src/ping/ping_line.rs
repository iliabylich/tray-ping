#[derive(Debug, Clone)]
pub(crate) struct PingLine {
    hostname: String,
    icmp_seq: u64,
    ttl: u32,
    duration: std::time::Duration,
}

pub(crate) type PingResult = Result<PingLine, String>;

impl std::fmt::Display for PingLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "64 bytes from {}: icmp_seq={} ttl={} time={}ms",
            self.hostname,
            self.icmp_seq,
            self.ttl,
            self.duration.as_millis(),
        )
    }
}

impl PingLine {
    pub(crate) fn new(hostname: String, ip_addr: std::net::IpAddr, icmp_seq: u64) -> PingResult {
        let ttl = 64;
        let start = std::time::Instant::now();

        ping::dgramsock::ping(ip_addr, None, Some(ttl), None, None, None)
            .map_err(|err| err.to_string())?;

        let duration = start.elapsed();

        Ok(Self {
            hostname: hostname.to_string(),
            icmp_seq,
            ttl,
            duration,
        })
    }
}

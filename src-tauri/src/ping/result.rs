#[derive(Debug, Clone)]
pub(crate) enum PingResult {
    Done {
        hostname: String,
        icmp_seq: u64,
        ttl: u32,
        duration: std::time::Duration,
    },
    Error(String),
}

impl std::fmt::Display for PingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Done {
                hostname,
                icmp_seq,
                ttl,
                duration,
            } => {
                write!(
                    f,
                    "64 bytes from {}: icmp_seq={} ttl={} time={}ms",
                    hostname,
                    icmp_seq,
                    ttl,
                    duration.as_millis(),
                )
            }
            Self::Error(e) => {
                write!(f, "Error: {}", e)
            }
        }
    }
}

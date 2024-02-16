use std::net::ToSocketAddrs;

#[derive(Debug)]
pub(crate) struct DnsError;

impl std::fmt::Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DNS lookup error")
    }
}

impl std::error::Error for DnsError {}

pub(crate) fn hostname_to_ip_addr(hostname: &str) -> Result<std::net::IpAddr, DnsError> {
    match hostname.parse() {
        Ok(ip_addr) => Ok(ip_addr),
        Err(_) => match hostname.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(addr) => Ok(addr.ip()),
                None => Err(DnsError),
            },
            Err(_) => Err(DnsError),
        },
    }
}

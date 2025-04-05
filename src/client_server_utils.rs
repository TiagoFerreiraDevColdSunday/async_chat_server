use get_if_addrs::get_if_addrs;

/// Get the first non-loopback IPv4 address of the machine
pub fn get_machine_ip() -> Option<String> {
    let if_addrs = get_if_addrs().ok()?;
    for iface in if_addrs {
        if !iface.is_loopback() {
            if let std::net::IpAddr::V4(ip) = iface.ip() {
                return Some(ip.to_string());
            }
        }
    }
    None
}

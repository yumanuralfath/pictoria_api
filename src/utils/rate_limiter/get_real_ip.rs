use rocket::Request;
use std::net::IpAddr;
use std::str::FromStr;

pub fn get_real_ip(req: &Request<'_>) -> Option<IpAddr> {
    if let Some(ip_str) = req.headers().get_one("CF-Connecting-IP") {
        if let Ok(ip) = IpAddr::from_str(ip_str) {
            return Some(ip);
        }
    }

    if let Some(ip_str) = req.headers().get_one("X-Forwarded-For") {
        if let Some(first_ip) = ip_str.split(',').next() {
            if let Ok(ip) = IpAddr::from_str(first_ip.trim()) {
                return Some(ip);
            }
        }
    }

    req.client_ip()
}

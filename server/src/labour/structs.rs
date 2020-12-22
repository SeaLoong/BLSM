use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ConnectionInfo {
    peer_addr: SocketAddr,
    scheme: String,
    host: String,
    real_remote_addr: Option<SocketAddr>,
    remote_addr: Option<SocketAddr>,
}

impl ConnectionInfo {
    pub fn new(info: &actix_web::dev::ConnectionInfo, peer_addr: SocketAddr) -> ConnectionInfo {
        ConnectionInfo {
            peer_addr,
            scheme: String::from(info.scheme()),
            host: String::from(info.host()),
            real_remote_addr: info
                .realip_remote_addr()
                .and_then(|s| SocketAddr::from_str(s).ok()),
            remote_addr: info
                .remote_addr()
                .and_then(|s| SocketAddr::from_str(s).ok()),
        }
    }
}

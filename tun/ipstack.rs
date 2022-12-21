use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum State {
    NoState,
    InUseOtherUser,
    NeedsLogin,
    NeedsMachineAuth,
    Stopped,
    Starting,
    Running,
}

struct IPN {
    state: State,
    local_ip: IpAddr,
    dns_ip: IpAddr,
    ip_map: HashMap<IpAddr, IpAddr>,
}

impl IPN {
    fn new() -> Self {
        Self {
            state: State::NoState,
            local_ip: "0.0.0.0".parse().unwrap(),
            dns_ip: "0.0.0.0".parse().unwrap(),
            ip_map: HashMap::new(),
        }
    }

    fn up(&mut self, conf: &Conf) {
        self.local_ip = conf.local_ip;
        self.dns_ip = conf.dns_ip;
        self.ip_map = conf.ip_map;
    }

    fn down(&mut self) {
        self.local_ip = "0.0.0.0".parse().unwrap();
        self.dns_ip = "0.0.0.0".parse().unwrap();
        self.ip_map = HashMap::new();
    }

    fn state(&self) -> State {
        self.state
    }
}

struct Conf {
    local_ip: IpAddr,
    dns_ip: IpAddr,
    ip_map: HashMap<IpAddr, IpAddr>,
}

impl Conf {
    fn new(local_ip: IpAddr, dns_ip: IpAddr, ip_map: HashMap<IpAddr, IpAddr>) -> Self {
        Self {
            local_ip,
            dns_ip,
            ip_map,
        }
    }
}

struct IpStack {
    ipn: Arc<IPN>,
}

impl IpStack {
    fn new() -> Self {
        Self {
            ipn: Arc::new(IPN::new()),
        }
    }

    fn init(&self) {}

    fn connect(&self, addr: SocketAddr) {}

    fn listen(&self, addr: SocketAddr) {}

    fn bind(&self, addr: SocketAddr) {}

    fn parse_ip(&self, s: &str) -> Result<IpAddr, std::net::AddrParseError> {
        s.parse()
    }

    fn resolve(&self, host: &str) -> Result<IpAddr, std::io::Error> {
        Ok(IpAddr::from(0))
    }

    fn up(&self, conf: &Conf) {
        let mut ipn = self.ipn.clone();
        ipn.up(conf);

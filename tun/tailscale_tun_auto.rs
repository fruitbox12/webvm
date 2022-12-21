use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

#[wasm_bindgen]
extern "C" {
    fn listen(a: u32, b: u32, c: u32) -> u32;
    fn connect(a: u32, b: u32, c: u32) -> u32;
    fn bind(a: u32, b: u32, c: u32) -> u32;
    fn parse_ip(a: u32, b: u32, c: u32) -> u32;
    fn resolve(a: u32, b: u32, c: u32) -> u32;
    fn up(a: u32, b: u32) -> u32;
    fn down() -> u32;
    fn login() -> u32;
    fn logout() -> u32;
    fn init() -> u32;
}

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
            local_ip: "0.0.0.0".parse().

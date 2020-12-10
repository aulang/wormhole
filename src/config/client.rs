use super::NetAddr;

#[derive(Debug)]
pub struct Client {
    key: String,
    server_addr: NetAddr,
}

impl Client {
    pub fn new(key: String, server_addr: NetAddr) -> Client {
        Client { key, server_addr }
    }
}

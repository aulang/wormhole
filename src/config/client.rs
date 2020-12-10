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

    pub fn get_key(&self) -> &String {
        &(self.key)
    }

    pub fn get_addr(&self) -> &NetAddr {
        &(self.server_addr)
    }
}

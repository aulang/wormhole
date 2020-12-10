use super::NetAddr;

#[derive(Debug)]
pub struct Client {
    key: String,
    server_addr: NetAddr,
    local_addrs: Vec<NetAddr>,
    max_tunnel: u32,
}

impl Client {
    pub fn new(
        key: String,
        server_addr: NetAddr,
        local_addrs: Vec<NetAddr>,
        max_tunnel: u32,
    ) -> Client {
        Client {
            key,
            server_addr,
            local_addrs,
            max_tunnel,
        }
    }

    pub fn get_key(&self) -> &String {
        &(self.key)
    }

    pub fn get_addr(&self) -> &NetAddr {
        &(self.server_addr)
    }
}

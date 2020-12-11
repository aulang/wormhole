//! # 客户端配置

use super::NetAddr;

#[derive(Debug)]
pub struct Client {
    key: String,               // 连接密钥
    server_addr: NetAddr,      // 服务器地址
    local_addrs: Vec<NetAddr>, // 本地代理地址和服务器代理端口
    max_tunnel: u32,           // 连接最大通道数，范围[1, 10]
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

    pub fn get_server_addr(&self) -> &NetAddr {
        &(self.server_addr)
    }

    pub fn get_local_addrs(&self) -> &Vec<NetAddr> {
        &(self.local_addrs)
    }

    pub fn get_max_tunnel(&self) -> u32 {
        self.max_tunnel
    }

    pub fn parse_args(args: &Vec<String>) -> Result<Client, &str> {
        let len = args.len();

        if len < 3 {
            return Err("参数缺失！");
        }

        for arg in args.iter() {
            println!("参数：{}", arg);
        }

        if let Some(x) = args.get(0) {
            println!("参数：{}", x);
        }

        let key: String = String::from("");
        let server_addr: NetAddr = NetAddr::new(String::from(""), 0, 0);
        let local_addrs: Vec<NetAddr> = vec![];
        let max_tunnel: u32 = 2;

        Ok(Client {
            key,
            server_addr,
            local_addrs,
            max_tunnel,
        })
    }
}

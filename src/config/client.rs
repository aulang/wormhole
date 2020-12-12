//! # 客户端配置

use super::net_addr::NetAddr;

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

    pub fn parse_args(args: &[String]) -> Result<Client, String> {
        let len = args.len();

        if len < 2 {
            return Err(format!("参数缺失：{:?}！", args));
        }

        let key: String = args[0].clone();

        let server_addr: NetAddr = NetAddr::parse(&args[1])?;

        let local_addrs: Vec<NetAddr> = NetAddr::parse_array(&args[2]);
        
        if local_addrs.is_empty() {
            return Err(format!("地址格式错误：{}！", args[2]));
        }

        let mut max_tunnel = 1;

        if let Some(arg) = args.get(3) {
            if let Ok(i) = arg.parse::<u32>() {
                max_tunnel = i;
            } else {
                eprintln!("通道数格式不对：{}", arg);
            }
        }

        Ok(Client {
            key,
            server_addr,
            local_addrs,
            max_tunnel,
        })
    }
}

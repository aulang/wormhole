//! # 配置模块

pub mod client;
pub mod server;

/// 网络地址
/// addr：IPv4地址
/// port：端口号
/// proxy_port：代理端口
#[derive(Debug)]
pub struct NetAddr {
    pub addr: String,    // IPv4地址
    pub port: u32,       // 端口号
    pub proxy_port: u32, // 代理端口
}

impl NetAddr {
    pub fn new(addr: String, port: u32, proxy_port: u32) -> NetAddr {
        NetAddr {
            addr,
            port,
            proxy_port,
        }
    }
}

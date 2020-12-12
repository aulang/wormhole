//! # 配置模块

pub mod client;
pub mod net_addr;
pub mod server;

use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use toml::value::Array;

#[derive(Debug, Deserialize)]
struct Server {
    key: String,
    port: u32,
    min_proxy_port: u32,
    max_proxy_port: u32,
}

#[derive(Debug, Deserialize)]
struct Client {
    key: String,
    server_addr: String,
    local_addrs: Array,
    max_tunnel: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    server: Server,
    client: Client,
}

pub fn read_config(file_path: &str) -> Result<Config, String> {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("打开配置文件失败：{}", e)),
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Ok(s) => s,
        Err(e) => return Err(format!("读取配置文件失败：{}", e)),
    };

    let config = match toml::from_str::<Config>(&file_content) {
        Ok(c) => c,
        Err(e) => return Err(format!("解析配置文件失败：{}", e)),
    };

    Ok(config)
}

pub fn read_client_config(file_path: &str) -> Result<client::Client, String> {
    let client = read_config(file_path)?.client;

    let server_addr = net_addr::NetAddr::parse(&client.server_addr)?;

    let mut local_addrs: Vec<net_addr::NetAddr> = Vec::new();

    for local_addr in client.local_addrs.iter() {
        if let Some(s) = local_addr.as_str() {
            match net_addr::NetAddr::parse(s) {
                Ok(n) => local_addrs.push(n),
                Err(e) => eprintln!("地址格式错误{}！", e),
            };
        }
    }

    if local_addrs.is_empty() {
        return Err(format!("地址格式错误：{:?}！", client.local_addrs));
    }

    Ok(client::Client::new(
        client.key,
        server_addr,
        local_addrs,
        client.max_tunnel,
    ))
}

pub fn read_server_config(file_path: &str) -> Result<server::Server, String> {
    let server = read_config(file_path)?.server;
    Ok(server::Server::new_full(
        server.key,
        server.port,
        server.min_proxy_port,
        server.max_proxy_port,
    ))
}

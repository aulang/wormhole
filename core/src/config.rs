use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Deserialize)]
#[structopt(name = "server")]
pub struct Server {
    #[structopt(short = "k", long = "key")]
    pub key: String,
    #[structopt(short = "p", long = "port")]
    pub port: u32,
    #[structopt(default_value = "1025")]
    pub min_proxy_port: u32,
    #[structopt(default_value = "65535")]
    pub max_proxy_port: u32,
}

impl Server {
    pub fn parse_args() -> Server {
        return Server::from_args();
    }
}

#[derive(Debug, StructOpt, Deserialize)]
#[structopt(name = "client")]
pub struct Client {
    #[structopt(short = "k", long = "key")]
    pub key: String,
    #[structopt(short = "s", long = "server")]
    pub server_addr: String,
    #[structopt(short = "p", long = "proxy")]
    pub local_addrs: Vec<String>,
    #[structopt(default_value = "1")]
    pub max_tunnel: u32,
}

impl Client {
    pub fn parse_args() -> Client {
        return Client::from_args();
    }
}

pub fn read_client_config(file_path: &str) -> Result<Client, String> {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("打开配置文件失败：{}", e)),
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Ok(s) => s,
        Err(e) => return Err(format!("读取配置文件失败：{}", e)),
    };

    let client = match toml::from_str::<Client>(&file_content) {
        Ok(o) => o,
        Err(e) => return Err(format!("解析配置文件失败：{}", e)),
    };

    Ok(client)
}

pub fn read_server_config(file_path: &str) -> Result<Server, String> {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("打开配置文件失败：{}", e)),
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Ok(s) => s,
        Err(e) => return Err(format!("读取配置文件失败：{}", e)),
    };

    let server = match toml::from_str::<Server>(&file_content) {
        Ok(o) => o,
        Err(e) => return Err(format!("解析配置文件失败：{}", e)),
    };

    Ok(server)
}

use std::env;

use config::client::Client;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("参数错误！");
    }

    match Client::parse_args(&args[2..]) {
        Ok(client) => println!("启动参数：{:#?}", client),
        Err(err) => println!("错误信息：{}", err),
    }

    match config::read_config("D:/Project/Rust/wormhole/config.toml") {
        Ok(config) => println!("配置文件：{:#?}", config),
        Err(err) => println!("错误信息：{}", err),
    }

    match config::read_client_config("D:/Project/Rust/wormhole/config.toml") {
        Ok(client) => println!("配置文件client：{:#?}", client),
        Err(err) => println!("错误信息：{}", err),
    }

    match config::read_server_config("D:/Project/Rust/wormhole/config.toml") {
        Ok(server) => println!("配置文件server：{:#?}", server),
        Err(err) => println!("错误信息：{}", err),
    }
}

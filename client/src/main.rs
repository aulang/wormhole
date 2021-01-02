mod config;

fn main() {
    let client =
        config::Client::read_config("D:/Project/Rust/wormhole/client/config.toml");

    println!("配置文件client：{:#?}", client);
}

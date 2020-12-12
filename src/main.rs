mod config;

use config::client::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let result = Client::parse_args(&args[2..]);

    match result {
        Ok(client) => println!("{:#?}", client),
        Err(err) => println!("错误信息：{}", err),
    }
}

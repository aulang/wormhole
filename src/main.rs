mod config;

use config::client::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("size = {}", args.len());
    println!("{:?}", args);

    let addr = config::NetAddr {
        addr: String::from("127.0.0.1"),
        port: 8080,
    };

    let client = Client::new(String::from("Aulang"), addr);

    println!("{:#?}", client);
}

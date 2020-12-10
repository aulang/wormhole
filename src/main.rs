mod config;

use config::client::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("size = {}", args.len());
    println!("{:?}", args);

    let server_addr = config::NetAddr {
        addr: String::from("127.0.0.1"),
        port: 8080,
    };

    let local_addrs = vec![config::NetAddr {
        addr: String::from("127.0.0.1"),
        port: 8080,
    }];
    
    let max_tunnel = 2;

    let client = Client::new(String::from("Aulang"), server_addr, local_addrs, max_tunnel);

    println!("{:#?}", client);
}

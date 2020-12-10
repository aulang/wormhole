pub mod client;
pub mod server;

#[derive(Debug)]
pub struct NetAddr {
    pub addr: String,
    pub port: u32,
}

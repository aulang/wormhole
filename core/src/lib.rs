pub mod config;
pub mod net_addr;
pub mod protocol;

#[cfg(test)]
mod tests {
    use crate::config;

    #[test]
    fn read_client_config() {
        let client =
            config::read_client_config("D:/Project/Rust/wormhole/client/config.toml").unwrap();

        println!("{:#?}", client);
    }

    #[test]
    fn read_server_config() {
        let server =
            config::read_server_config("D:/Project/Rust/wormhole/server/config.toml").unwrap();

        println!("{:#?}", server);
    }
}

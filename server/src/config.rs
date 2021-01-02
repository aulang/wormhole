//! # 服务端端配置

#[derive(Debug)]
pub struct Server {
    key: String,         // 连接密钥
    port: u32,           // 监听端口
    min_proxy_port: u32, // 代理端口范围，最小代理端口，1025
    max_proxy_port: u32, // 代理端口范围，最大代理端口，65535
}

impl Server {
    pub fn new(key: String, port: u32) -> Server {
        Server {
            key: key,
            port: port,
            min_proxy_port: 1025,
            max_proxy_port: 65535,
        }
    }

    pub fn new_full(key: String, port: u32, min_proxy_port: u32, max_proxy_port: u32) -> Server {
        let (mut min_proxy_port, mut max_proxy_port) = (max_proxy_port, min_proxy_port);

        if min_proxy_port < 1025 || min_proxy_port > 65535 {
            min_proxy_port = 1025;
        }

        if max_proxy_port < 1025 || max_proxy_port > 65535 {
            max_proxy_port = 65535;
        }

        if min_proxy_port > max_proxy_port {
            std::mem::swap(&mut min_proxy_port, &mut max_proxy_port);
        }

        Server {
            key: key,
            port: port,
            min_proxy_port,
            max_proxy_port,
        }
    }

    pub fn get_key(&self) -> &String {
        &(self.key)
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    pub fn get_min_proxy_port(&self) -> u32 {
        self.min_proxy_port
    }

    pub fn get_max_proxy_port(&self) -> u32 {
        self.max_proxy_port
    }

    pub fn parse_args() -> Server {
        let server = core::config::Server::parse_args();

        Server::new_full(
            server.key,
            server.port,
            server.min_proxy_port,
            server.max_proxy_port,
        )
    }

    pub fn read_config(file_path: &str) -> Server {
        let server = match core::config::read_server_config(file_path) {
            Ok(o) => o,
            Err(e) => panic!(e),
        };

        Server::new_full(
            server.key,
            server.port,
            server.min_proxy_port,
            server.max_proxy_port,
        )
    }
}

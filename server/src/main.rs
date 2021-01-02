use std::env;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = if args.len() > 1 {
        config::Server::parse_args()
    } else {
        let config_path = env::current_exe()
            .expect("配置文件不存在！")
            .with_file_name("config.toml");
        config::Server::read_config(config_path.to_str().expect("配置文件不存在！"))
    };

    println!("Server配置：{:#?}", server);
}

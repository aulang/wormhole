/// 网络地址
/// addr：IPv4地址
/// port：端口号
/// proxy_port：代理端口
#[derive(Debug)]
pub struct NetAddr {
    pub addr: String,    // IPv4地址
    pub port: u32,       // 端口号
    pub proxy_port: u32, // 代理端口
}

impl NetAddr {
    pub fn new(addr: String, port: u32, proxy_port: u32) -> NetAddr {
        NetAddr {
            addr,
            port,
            proxy_port,
        }
    }

    pub fn parse(address: &str) -> Result<NetAddr, String> {
        let strs: Vec<&str> = address.split(":").collect();

        if strs.len() < 2 {
            return Err(format!("地址格式不对：{}！", address));
        }

        let addr = String::from(strs[0]);

        let mut port = 0;

        let s1 = String::from(strs[1]);
        let p1 = s1.parse::<u32>();
        if let Ok(i) = p1 {
            port = i;
        } else {
            return Err(format!("端口格式不对：{}！", s1));
        }

        let mut proxy_port = 0;

        if let Some(s2) = strs.get(2) {
            let p2 = s2.parse::<u32>();
            if let Ok(i) = p2 {
                proxy_port = i;
            } else {
                return Err(format!("端口格式不对：{}！", s2));
            }
        }

        return Ok(NetAddr::new(addr, port, proxy_port));
    }

    pub fn parse_array(addresses: &String) -> Vec<NetAddr> {
        let mut addrs: Vec<NetAddr> = Vec::new();

        let addresses: Vec<&str> = addresses.split(",").collect();

        for address in addresses.iter() {
            if let Ok(addr) = NetAddr::parse(address) {
                addrs.push(addr)
            } else {
                eprintln!("地址格式不对：{}", address);
            }
        }

        return addrs;
    }
}

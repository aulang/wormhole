pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Server {
        Server { port }
    }
}

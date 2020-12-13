//! 连接协议

pub struct Protocol {
    result: u8,
    version: u8,
    port: u16,
    key: String,
}

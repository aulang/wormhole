/// 连接协议
/// result：处理结果
/// version：协议版本
/// port：代理端口
/// key：连接密钥
pub struct Protocol {
    result: u8,
    version: u8,
    port: u16,
    key: String,
}
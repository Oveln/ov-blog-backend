use config::{Config, File};
use lazy_static::lazy_static;
use serde::Deserialize;

// 应用配置
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    // 应用监听地址
    pub host: String,
    // 应用监听端口
    pub port: u16,
    // 数据库连接地址
    pub db_uri: String,
    // 数据库名称
    pub db_name: String,
}

lazy_static! {
    pub static ref CONFIG: AppConfig = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("需要配置文件config.toml")
        .try_deserialize()
        .expect("配置文件解析错误");
}

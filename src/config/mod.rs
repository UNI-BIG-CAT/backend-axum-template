pub mod rsa_key;
use crate::constant::APP_ENV;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
/* *******************************************************************
 * 配置结构体
 ******************************************************************* */
// 主redis配置
#[derive(Debug, Deserialize, Clone)] // #[derive(...)] 是派生宏（derive macro），会生成代码。
pub struct MainRedis {
    pub ip: String,
    pub port: String,
    pub auth: String,
    pub db: i32,
}
// postgres配置
#[derive(Debug, Deserialize, Clone)]
pub struct Postgres {
    pub ip: String,
    pub port: String,
    pub db: String,
    pub admin: String,
    pub password: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub sqlx_logging: bool,
}
// 用户配置
#[derive(Debug, Deserialize, Clone)] // #[derive(...)] 是派生宏（derive macro），会生成代码。
pub struct Admin {
    #[serde(default = "default_expires_in")]
    pub expires_in: u32, // 用户token过期时间 单位：天
}
// 配置结构体
#[derive(Debug, Deserialize, Clone)] // #[derive(...)] 是派生宏（derive macro），会生成代码。
pub struct Config {
    // #[serde(...)] 是一个 属性宏参数，但写在结构体字段上，作用是告诉 serde 如何处理这个字段。
    #[serde(default = "default_http_port")]
    pub http_port: u16, // 默认端口
    #[serde(default = "default_log_level")]
    pub log_level: String, // 默认日志级别
    #[serde(default = "default_log_file")]
    pub log_file: String, // 默认日志文件
    #[serde(default = "default_save_log")]
    pub save_log: bool, // 默认是否保存日志
    pub main_redis: MainRedis,
    pub postgres: Postgres,
    pub admin: Admin,
}

// 默认配置
fn default_http_port() -> u16 {
    3000
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_file() -> String {
    "logs/app.log".to_string()
}
fn default_save_log() -> bool {
    false
}
fn default_expires_in() -> u32 {
    7 // 用户token过期时间 单位：天
}
/* *******************************************************************
 * 实现 Config 结构体
 ******************************************************************* */
impl Config {
    pub fn from_env() -> Self {
        // 读取环境变量
        let env = env::var(APP_ENV).unwrap_or_else(|_| "dev".to_string());
        // 根据环境变量选择文件
        let file_name = match env.as_str() {
            "pro" => "production.yaml",
            _ => "develop.yaml",
        };
        // 读取配置文件
        let path = Path::new("config").join(file_name);
        // 读取配置文件内容
        let content = fs::read_to_string(path).expect("config read failed!");
        // 解析配置文件
        let config: Config = serde_yaml::from_str(&content).expect("config yaml read failed!");

        // 返回配置
        config
    }
}

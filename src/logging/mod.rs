use crate::config::Config;
use crate::constant::APP_ENV;
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use tracing::{debug, info};
use tracing_subscriber::fmt::writer::BoxMakeWriter;
// 初始化日志
pub fn init_logging(config: &Config) {
    let env_filter = format!("radar_detector={}", config.log_level);
    // 日志配置
    if config.save_log {
        // 创建日志目录
        let log_dir = Path::new(&config.log_file).parent().unwrap();
        if !log_dir.exists() {
            std::fs::create_dir_all(log_dir).unwrap();
        }
        // 打开或创建日志文件
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.log_file)
            .unwrap();

        // 日志写入文件
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(BoxMakeWriter::new(file))
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    } else {
        // 日志输出到控制台
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }
    // 打印初始日志
    // 读取环境变量
    let env = env::var(APP_ENV).unwrap_or_else(|_| "dev".to_string());
    info!("RADAR DETECTOR BACKEND STARTED!!!");
    info!("CURRENT ENVIRONMENT: {}", env);
    info!("HTTP PORT: {:?}", &config.http_port);
    debug!("CONFIG DETAIL: {:?}", &config);
}

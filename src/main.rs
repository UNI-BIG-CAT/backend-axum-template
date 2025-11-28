// 导入自定义模块
mod config;
mod constant;
mod controllers;
mod dto;
mod logging;
mod middleware;
mod repository;
mod service;

// 导入依赖
use crate::config::Config;
use controllers::get_router;

/* ********************** 主函数 ********************** */
#[tokio::main] // 使用 tokio 运行异步代码
async fn main() {
    // 获取配置
    let config = Config::from_env();
    let http_port = config.http_port;

    // 初始化日志
    logging::init_logging(&config);

    // 初始化数据库
    let app_state = middleware::app_state::get_app_state(config).await;

    // 构建应用路由
    let app = get_router(app_state);

    // 启动应用
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", http_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

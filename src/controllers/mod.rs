// 导入子模块
pub mod admin;
// 导入中间件
use crate::middleware::app_middleware::{
    auth_middleware, cors_layer, error_handler_middleware, logging_middleware, trace_layer,
};
use crate::middleware::app_state::AppState;

use axum::{
    Router, middleware,
    routing::{get, post},
};

// 获取路由
pub fn get_router(app_state: AppState) -> Router {
    // 分别创建路由器
    let no_auth_router = Router::new()
        .route("/admin/register", post(admin::register))
        .route("/admin/activeEmailCode", get(admin::active_email_code))
        .route("/admin/login", post(admin::login));

    let admin_router = Router::new()
        .route("/admin/my", get(admin::my))
        .route("/admin/logout", post(admin::logout))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // 合并所有路由，并添加全局中间件
    Router::new()
        .merge(no_auth_router)
        .merge(admin_router)
        .layer(middleware::from_fn(error_handler_middleware)) // 全局错误处理
        .layer(cors_layer()) // 全局 CORS
        .layer(trace_layer()) // 全局 Trace
        .layer(middleware::from_fn(logging_middleware)) // 给所有路由加日志
        .with_state(app_state.clone())
}

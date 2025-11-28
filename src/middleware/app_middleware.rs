use crate::middleware::{app_response::ApiResponse, app_state::AppState};
use crate::service::auth::Jwttoken;
use axum::extract::FromRequest;
use axum::http::header;
use axum::response::Response;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use serde::de::DeserializeOwned;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, info_span};
use uuid::Uuid;
use validator::Validate;
/**************************************************************************************************
 * 中间件
 **************************************************************************************************/
//  权限监测中间件
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    debug!("headers: {:?}", headers);

    let auth_str_opt = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").trim().to_string());

    let Some(auth_str) = auth_str_opt else {
        debug!("Authorization header missing or invalid");
        let body: ApiResponse<(i32, String)> = ApiResponse::error(401);
        return body.into_response();
    };
    debug!("Extracted auth token: {}", auth_str);

    let Some(jwt_info) = Jwttoken::verify_jwt(&auth_str, &state.rsa_key.jwt_public) else {
        error!("JWT verification failed for token: {}", auth_str);
        let body: ApiResponse<(i32, String)> = ApiResponse::error(401);
        return body.into_response();
    };
    debug!("JWT verified successfully: {:?}", jwt_info);

    request.extensions_mut().insert(jwt_info);

    next.run(request).await
}

// 日志中间件
pub async fn logging_middleware(request: Request, next: axum::middleware::Next) -> Response {
    let start = Instant::now();
    let headers = request.headers().clone();
    let req_id = Uuid::new_v4().to_string();
    // 创建 span，并绑定 request_id
    let span = info_span!("request", %req_id, method = %request.method(), path = %request.uri());
    let _enter = span.enter();
    // 打印请求头
    if let Some(user_agent) = headers.get("user-agent") {
        info!(
            "[req start][User-Agent:{}]",
            user_agent.to_str().unwrap_or("unknown")
        );
    } else {
        info!("[req start]");
    }
    // 执行下一个中间件
    let response = next.run(request).await;
    // 打印请求结束
    let duration = start.elapsed();
    info!("[req end]::{}ms", duration.as_millis());
    // 返回响应
    response
}

// 错误处理中间件
pub async fn error_handler_middleware(request: Request, next: axum::middleware::Next) -> Response {
    use futures::FutureExt;
    let fut = std::panic::AssertUnwindSafe(next.run(request)).catch_unwind();

    match fut.await {
        Ok(response) => response,
        Err(panic_info) => {
            // 尝试 downcast 成 &str 或 String
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            // 捕获 panic，打印日志
            error!("server error:: {:?}", panic_msg);
            let body: ApiResponse<(i32, String)> = ApiResponse::error(500);
            body.into_response()
        }
    }
}

// 创建 CORS 中间件
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

// 创建 Trace 中间件
pub fn trace_layer()
-> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
}

/**************************************************************************************************
 * 自定义验证 Extractor
 **************************************************************************************************/
/// 带验证的 Json Extractor
/// 使用方式：在 handler 中使用 `ValidatedJson<RegisterRequest>` 替代 `Json<RegisterRequest>`
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 提取 JSON body
        let axum::Json(body) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|_| {
                let error_response: ApiResponse<()> = ApiResponse::error(405);
                error_response.into_response()
            })?;

        // 验证
        if let Err(errors) = body.validate() {
            let error_code = errors
                .field_errors()
                .values()
                .next()
                .and_then(|errors_vec| errors_vec.first())
                .and_then(|error| error.message.as_ref())
                .and_then(|msg| msg.parse::<u32>().ok())
                .unwrap_or(406);
            let error_response: ApiResponse<()> = ApiResponse::error(error_code);
            return Err(error_response.into_response());
        }

        Ok(ValidatedJson(body))
    }
}

/// 带验证的 Query Extractor
/// 使用方式：在 handler 中使用 `ValidatedQuery<ActiveEmailCodeRequest>` 替代 `Query<ActiveEmailCodeRequest>`
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 提取 Query 参数
        let axum::extract::Query(query) = axum::extract::Query::<T>::from_request(req, state)
            .await
            .map_err(|_| {
                let error_response: ApiResponse<()> = ApiResponse::error(405);
                error_response.into_response()
            })?;

        // 验证
        if let Err(errors) = query.validate() {
            let error_code = errors
                .field_errors()
                .values()
                .next()
                .and_then(|errors_vec| errors_vec.first())
                .and_then(|error| error.message.as_ref())
                .and_then(|msg| msg.parse::<u32>().ok())
                .unwrap_or(406);
            let error_response: ApiResponse<()> = ApiResponse::error(error_code);
            return Err(error_response.into_response());
        }

        Ok(ValidatedQuery(query))
    }
}

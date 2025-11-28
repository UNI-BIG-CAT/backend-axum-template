use crate::constant::ADMIN_ENABLED;
use crate::dto::admin::*;
use crate::middleware::app_middleware::{ValidatedJson, ValidatedQuery};
use crate::middleware::app_response::ApiResponse;
use crate::middleware::app_state::AppState;
use crate::repository::redis::mode::*;
use crate::service::admin::AdminService;
use crate::service::auth::{AuthService, JwtPayload, Jwttoken};
use axum::{
    extract::{Extension, State},
    response::IntoResponse,
};
/**************************************************************************************************
 * 注册
 **************************************************************************************************/
#[utoipa::path(
    post,
    path = "/admin/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Success",body = RegisterResponse)
    )
)]
#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(params): ValidatedJson<RegisterRequest>,
) -> impl IntoResponse {
    // 1、处理密码加密
    let pw: String = params.password;
    let new_passwort = AuthService::hmac_sha(&pw, &state.rsa_key.pw_private);
    // 2、写入数据库
    let model = AdminService::create(
        &state.db,
        1,
        params.admin_name,
        new_passwort,
        params.email.to_string(),
        params.phone.to_string(),
    )
    .await
    .unwrap();
    // 3、生成邮箱验证码
    let code = uuid::Uuid::new_v4().to_string();
    // 4、写入缓存
    let cache_key = format!("{}{}", ADMIN_EMAIL_CODE_PREFIX, model.admin_id);
    let email_cache = EmailCodeCache {
        email: model.email.clone(),
        code: code.clone(),
    };
    state
        .main_redis
        .set_ex(&cache_key, &email_cache, 60 * 60 * 24)
        .await;
    // 5、返回结果
    let response = RegisterResponse {
        admin_id: model.admin_id,
        admin_name: model.admin_name,
        email: model.email,
        phone: model.phone,
        role_id: model.role_id,
    };
    ApiResponse::success(response)
}

#[utoipa::path(
    post,
    path = "/admin/activeEmailCode",
    request_body = ActiveEmailCodeRequest,
    responses(
        (status = 200, description = "Success",body = LoginResponse)
    )
)]
#[axum::debug_handler]
pub async fn active_email_code(
    State(state): State<AppState>,
    ValidatedQuery(query): ValidatedQuery<ActiveEmailCodeRequest>,
) -> impl IntoResponse {
    let admin_id = query.admin_id;
    // 获取缓存中的验证码
    let cache_key = format!("{}{}", ADMIN_EMAIL_CODE_PREFIX, admin_id);
    if let Some(code_cached) = state
        .main_redis
        .get::<EmailCodeCache>(cache_key.as_str())
        .await
    {
        if code_cached.code == query.code {
            // 激活用户
            AdminService::enable_admin_by_id(&state.db, admin_id, ADMIN_ENABLED)
                .await
                .unwrap();
            // 删除验证码缓存
            state.main_redis.del(cache_key.as_str()).await;
        } else {
            return ApiResponse::error(3003);
        }
    } else {
        return ApiResponse::error(3002);
    }
    // 获取用户信息
    let admin_info = AdminService::get_admin_by_id(&state.db, admin_id)
        .await
        .unwrap();
    if admin_info.is_none() {
        return ApiResponse::error(3001);
    }
    let admin = admin_info.unwrap();
    let admin_name = admin.admin_name;
    let email = admin.email;
    let phone = admin.phone;
    // 生成session token
    let token = uuid::Uuid::new_v4().to_string();
    // 写入用户Id与Token缓存
    let admin_id_token_cache = AdminIdTokenCache {
        admin_id,
        token: token.clone(),
    };
    let admin_cache = AdminCache {
        admin_id,
        role_id: admin.role_id,
        admin_name: admin_name.clone(),
        email: email.clone(),
        phone: phone.clone(),
    };
    AdminService::set_admin_cache(
        &state.main_redis,
        admin_id,
        &admin_id_token_cache,
        token.clone(),
        &admin_cache,
        state.config.admin.expires_in,
    )
    .await;
    // 创建 JwtPayload
    let jwt_payload = JwtPayload { admin_id, token };
    let jwt_token = Jwttoken::generate_jwt(jwt_payload, &state.rsa_key.jwt_private).unwrap();
    // 创建 LoginResponse
    let response = LoginResponse {
        admin_id,
        admin_name,
        role_id: admin.role_id,
        email,
        phone,
        jwt_token,
    };
    ApiResponse::success(response)
}
/**************************************************************************************************
 * 登录
 **************************************************************************************************/
#[utoipa::path(
    post,
    path = "/admin/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Success",body = LoginResponse)
    )
)]
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    axum::Json(params): axum::Json<LoginRequest>,
) -> impl IntoResponse {
    let email: String = params.email;
    let password = params.password;
    let new_passwort = AuthService::hmac_sha(&password, &state.rsa_key.pw_private);
    // 获取管理员
    let admin = AdminService::get_admin_by_email(&state.db, email.clone())
        .await
        .unwrap();
    if admin.is_none() {
        return ApiResponse::error(3001);
    }
    let admin = admin.unwrap();
    // 验证密码
    if admin.password != new_passwort {
        return ApiResponse::error(3004);
    }
    // 验证状态
    if admin.enabled != ADMIN_ENABLED {
        return ApiResponse::error(3005);
    }
    // 整理数据并生成token
    let token = uuid::Uuid::new_v4().to_string();
    let admin_id = admin.admin_id;
    let role_id = admin.role_id;
    let admin_name = admin.admin_name.clone();
    let email = admin.email.clone();
    let phone = admin.phone.clone();
    // 写入用户Id与Token缓存
    let admin_id_token_cache = AdminIdTokenCache {
        admin_id,
        token: token.clone(),
    };
    let admin_cache = AdminCache {
        admin_id,
        role_id: admin.role_id,
        admin_name: admin_name.clone(),
        email: email.clone(),
        phone: phone.clone(),
    };
    AdminService::set_admin_cache(
        &state.main_redis,
        admin_id,
        &admin_id_token_cache,
        token.clone(),
        &admin_cache,
        state.config.admin.expires_in,
    )
    .await;
    // 生成jwt token
    let jwt_payload = JwtPayload { admin_id, token };
    let jwt_token = Jwttoken::generate_jwt(jwt_payload, &state.rsa_key.jwt_private).unwrap();
    let response = LoginResponse {
        admin_id,
        admin_name: admin_name,
        role_id: role_id,
        email: email,
        phone: phone,
        jwt_token,
    };
    return ApiResponse::success(response);
}

/**************************************************************************************************
 * 我的信息
 **************************************************************************************************/
#[utoipa::path(
    get,
    path = "/admin/my",
    responses(
        (status = 200, description = "Success",body = LoginResponse)
    )
)]
#[axum::debug_handler]
pub async fn my(
    State(state): State<AppState>,
    Extension(jwt_info): Extension<Jwttoken>,
) -> impl IntoResponse {
    let token = jwt_info.payload.token;
    let admin_cache = AdminService::get_admin_cache(&state.main_redis, token).await;
    if admin_cache.is_none() {
        return ApiResponse::error(3001);
    }
    let admin_cache = admin_cache.unwrap();
    let response = AdminInfoResponse {
        admin_id: admin_cache.admin_id,
        admin_name: admin_cache.admin_name,
        email: admin_cache.email,
        phone: admin_cache.phone,
        role_id: admin_cache.role_id,
    };
    return ApiResponse::success(response);
}

/**************************************************************************************************
 * 退出
 **************************************************************************************************/
#[axum::debug_handler]
pub async fn logout(
    State(state): State<AppState>,
    Extension(jwt_info): Extension<Jwttoken>,
) -> impl IntoResponse {
    let admin_id = jwt_info.payload.admin_id;
    let token = jwt_info.payload.token;
    AdminService::delete_admin_cache(&state.main_redis, admin_id, token).await;
    return ApiResponse::success("退出成功");
}

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

static CN_PHONE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^1[3-9]\d{9}$").unwrap());

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if CN_PHONE_RE.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("phone"))
    }
}

///！ 注册
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "bigcat")]
    pub admin_name: String,

    #[validate(email(message = "802"))]
    #[schema(example = "bigcat@example.com")]
    pub email: String,

    #[schema(example = "123321")]
    pub password: String,

    #[validate(custom(function = "validate_phone", message = "801"))]
    #[schema(example = "13812345678")]
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegisterResponse {
    pub admin_id: i32,
    pub admin_name: String,
    pub role_id: i32,
    pub email: String,
    pub phone: String,
}

///！ 邮箱激活
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActiveEmailCodeRequest {
    #[schema(example = "1")]
    pub admin_id: i32,
    pub code: String,
}

///！登录
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "802"))]
    #[schema(example = "bigcat@example.com")]
    pub email: String,

    #[schema(example = "123456")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub admin_id: i32,
    pub admin_name: String,
    pub role_id: i32,
    pub email: String,
    pub phone: String,
    pub jwt_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AdminInfoResponse {
    pub admin_id: i32,
    pub admin_name: String,
    pub email: String,
    pub phone: String,
    pub role_id: i32,
}

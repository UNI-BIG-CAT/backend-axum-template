use serde::{Deserialize, Serialize};

// 邮箱激活码缓存前缀
pub const ADMIN_EMAIL_CODE_PREFIX: &str = "admin_email_code:";
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailCodeCache {
    pub email: String,
    pub code: String,
}

// 用户Id与Token缓存前缀
pub const ADMIN_ID_TOKEN_CACHE_PREFIX: &str = "admin_id_token:";
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdminIdTokenCache {
    pub admin_id: i32,
    pub token: String,
}

// 用户缓存前缀
pub const ADMIN_TOKEN_CACHE_PREFIX: &str = "admin_cache:";
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdminCache {
    pub admin_id: i32,
    pub role_id: i32,
    pub admin_name: String,
    pub email: String,
    pub phone: String,
}

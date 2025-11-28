use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};
use tracing::error;

#[derive(Clone, Debug)]
pub struct RedisService {
    pub pool: deadpool_redis::Pool,
}

impl RedisService {
    pub fn new(pool: deadpool_redis::Pool) -> Self {
        Self { pool }
    }

    /// 设置值，不带过期
    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> bool {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                error!("Redis pool get error: {:?}", e);
                return false;
            }
        };

        let value_str = match serde_json::to_string(value) {
            Ok(s) => s,
            Err(e) => {
                error!("Serialize value error: {:?}", e);
                return false;
            }
        };

        match conn.set::<_, _, ()>(key, value_str).await {
            Ok(_) => true,
            Err(e) => {
                error!("Redis set error: {:?}", e);
                false
            }
        }
    }

    /// 设置带 TTL（秒）的值
    pub async fn set_ex<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) -> bool {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                error!("Redis pool get error: {:?}", e);
                return false;
            }
        };

        let value_str = match serde_json::to_string(value) {
            Ok(s) => s,
            Err(e) => {
                error!("Serialize value error: {:?}", e);
                return false;
            }
        };

        match conn.set_ex::<_, _, ()>(key, value_str, ttl_secs).await {
            Ok(_) => true,
            Err(e) => {
                error!("Redis set_ex error: {:?}", e);
                false
            }
        }
    }

    /// 获取值，反序列化为泛型 T
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                error!("Redis pool get error: {:?}", e);
                return None;
            }
        };
        let data: Option<String> = conn.get(key).await.ok()?;
        data.and_then(|s| serde_json::from_str(&s).ok())
    }

    /// 删除 key
    pub async fn del(&self, key: &str) -> bool {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                error!("Redis pool get error: {:?}", e);
                return false;
            }
        };

        conn.del::<_, ()>(key).await.is_ok()
    }
}

use crate::config::{Config, rsa_key};
use crate::repository::redis::connect::RedisService;
use crate::repository::{connect_postgres, connect_redis};
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub db: DatabaseConnection,
    pub main_redis: RedisService,
    pub rsa_key: rsa_key::RsaKey,
}

pub async fn get_app_state(config: Config) -> AppState {
    let db = connect_postgres(&config).await;
    let main_redis_pool = connect_redis(&config).await;
    let main_redis = RedisService::new(main_redis_pool);
    let keys = rsa_key::get_rsa_key();

    let app_state = AppState {
        config,
        db,
        main_redis,
        rsa_key: keys,
    };
    app_state
}

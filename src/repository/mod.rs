pub mod entity;
pub mod redis;

use crate::config::Config;
use deadpool_redis::Pool as RedisPool;
use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;
use tracing::info;

pub async fn connect_postgres(config: &Config) -> DatabaseConnection {
    // 数据库连接
    let postgres_path = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=disable",
        config.postgres.admin,
        config.postgres.password,
        config.postgres.ip,
        config.postgres.port,
        config.postgres.db,
    );
    info!("postgres connect: {}", postgres_path);
    let mut opt = ConnectOptions::new(postgres_path);
    opt.max_connections(config.postgres.max_connections)
        .min_connections(config.postgres.min_connections)
        .connect_timeout(Duration::from_secs(10))
        .sqlx_logging(config.postgres.sqlx_logging) // disable SQLx logging
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(1800));

    let db = Database::connect(opt)
        .await
        .expect("failed to connect postgres");
    db
}

pub async fn connect_redis(config: &Config) -> RedisPool {
    // 主redis连接
    let redis_path = format!(
        "redis://:{}@{}:{}/{}",
        config.main_redis.auth, config.main_redis.ip, config.main_redis.port, config.main_redis.db
    );
    info!("redis connect: {}", redis_path);

    let redis_cfg = deadpool_redis::Config::from_url(redis_path);
    let main_redis = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();
    main_redis
}

use crate::repository::entity::tb_admin;
use crate::repository::redis::connect::RedisService;
use crate::repository::redis::mode::{
    ADMIN_ID_TOKEN_CACHE_PREFIX, ADMIN_TOKEN_CACHE_PREFIX, AdminCache, AdminIdTokenCache,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
#[derive(Debug, Clone)]
pub struct AdminService;

impl AdminService {
    /***************************************************************************************/
    // 缓存相关
    /***************************************************************************************/
    pub async fn set_admin_cache(
        main_redis: &RedisService,
        admin_id: i32,
        admin_id_token_cache: &AdminIdTokenCache,
        token: String,
        admin_cache: &AdminCache,
        expires_in_days: u32,
    ) -> bool {
        // 写入用户Id与Token缓存
        let cache_key = format!("{}{}", ADMIN_ID_TOKEN_CACHE_PREFIX, admin_id);
        main_redis
            .set_ex(
                &cache_key,
                admin_id_token_cache,
                expires_in_days as u64 * 24 * 60 * 60,
            )
            .await;
        // 写入用户缓存
        let cache_key = format!("{}{}", ADMIN_TOKEN_CACHE_PREFIX, token);
        main_redis
            .set_ex(
                &cache_key,
                admin_cache,
                expires_in_days as u64 * 24 * 60 * 60,
            )
            .await
    }

    pub async fn get_admin_cache(main_redis: &RedisService, token: String) -> Option<AdminCache> {
        let cache_key = format!("{}{}", ADMIN_TOKEN_CACHE_PREFIX, token);
        main_redis.get::<AdminCache>(&cache_key).await
    }

    pub async fn delete_admin_cache(
        main_redis: &RedisService,
        admin_id: i32,
        token: String,
    ) -> bool {
        let cache_key = format!("{}{}", ADMIN_ID_TOKEN_CACHE_PREFIX, admin_id);
        main_redis.del(&cache_key).await;
        let cache_key = format!("{}{}", ADMIN_TOKEN_CACHE_PREFIX, token);
        main_redis.del(&cache_key).await
    }
    /***************************************************************************************/
    // 数据库相关
    /***************************************************************************************/
    // // 获取管理员列表
    // pub async fn get_admin_list(
    //     db: &DatabaseConnection,
    // ) -> Result<Vec<tb_admin::Model>, sea_orm::DbErr> {
    //     tb_admin::Entity::find().all(db).await
    // }
    // 根据id获取管理员
    pub async fn get_admin_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<tb_admin::Model>, sea_orm::DbErr> {
        tb_admin::Entity::find_by_id(id).one(db).await
    }

    // 创建管理员
    pub async fn create(
        db: &DatabaseConnection,
        role_id: i32,
        admin_name: String,
        password: String,
        email: String,
        phone: String,
    ) -> Result<tb_admin::Model, sea_orm::DbErr> {
        let new_admin = tb_admin::ActiveModel {
            role_id: Set(role_id),
            admin_name: Set(admin_name),
            password: Set(password),
            email: Set(email),
            phone: Set(phone),
            ..Default::default()
        };
        let res = new_admin.insert(db).await?;
        Ok(res)
    }

    /// 激活管理员
    pub async fn enable_admin_by_id(
        db: &DatabaseConnection,
        admin_id: i32,
        enabled: i16,
    ) -> Result<(), sea_orm::DbErr> {
        let admin = tb_admin::Entity::find_by_id(admin_id).one(db).await?;
        if admin.is_none() {
            return Err(sea_orm::DbErr::RecordNotFound(
                "Admin not found".to_string(),
            ));
        }
        let mut admin = admin.unwrap().into_active_model();
        admin.enabled = Set(enabled);
        admin.update(db).await?;
        Ok(())
    }

    // 根据邮箱获取管理员
    pub async fn get_admin_by_email(
        db: &DatabaseConnection,
        email: String,
    ) -> Result<Option<tb_admin::Model>, sea_orm::DbErr> {
        tb_admin::Entity::find()
            .filter(tb_admin::Column::Email.eq(email))
            .one(db)
            .await
    }
}

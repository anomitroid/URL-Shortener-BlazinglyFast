use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use bb8_redis::{bb8, RedisConnectionManager};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
}

pub type RedisPool = bb8::Pool<RedisConnectionManager>;

pub async fn create_redis_pool() -> Result<RedisPool, bb8_redis::redis::RedisError> {
    let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url)?;
    bb8::Pool::builder().build(manager).await
}
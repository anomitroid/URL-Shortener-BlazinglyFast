use db::RedisPool;
use sqlx::PgPool;
use actix_web::{web, HttpServer, App};

pub mod db;
pub mod models;
pub mod handlers;
pub mod routes;
pub mod errors;

pub use models::UrlEntry;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool
}

pub async fn run() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

        let redis_pool = db::create_redis_pool().await.expect("Failed to create Redis pool");
        let state = AppState { db: pool, redis: redis_pool };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("localhost", port))?
    .run()
    .await
}
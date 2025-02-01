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
}

pub async fn run() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let state = AppState { db: pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}
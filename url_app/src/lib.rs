use actix_web::{web, App, HttpServer};
use dashmap::DashMap;
use std::sync::Arc;

mod handlers;
mod models;
mod routes;
// mod templates;
mod errors;

pub use models::UrlEntry;

#[derive(Debug, Clone)]
pub struct AppState {
    pub store: Arc<DashMap<String, UrlEntry>>,
}

pub async fn run() -> std::io::Result<()> {
    let state = AppState{
        store: Arc::new(DashMap::new()),
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("localhost:3000")?
    .run()
    .await
}
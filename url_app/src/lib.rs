use actix_web::{web, App, HttpServer};
use dashmap::DashMap;
use std::sync::Arc;

mod handlers;
mod models;
mod routes;
mod templates;

pub use models::UrlEntry;

pub struct AppState {
    pub store: Arc<DashMap<String, UrlEntry>>,
}

pub async fn run() -> std::io::Result<()> {
    let store = Arc::new(DashMap::new());
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                store: store.clone(),
            }))
            .configure(routes::configure)
    })
    .bind("localhost:3000")?
    .run()
    .await
}
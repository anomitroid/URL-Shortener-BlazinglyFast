pub mod templates;
pub mod handlers;
pub mod routes;

use actix_web::{web, App, HttpServer};
use std::io;

pub struct Application {
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(bind_address: &str) -> Result<Self, io::Error> {
        let templates = web::Data::new(templates::Templates::new());
        let server = HttpServer::new(move || {
            App::new()
                .app_data(templates.clone())
                .configure(routes::configure)
        })
        .bind(bind_address)?
        .run();

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), io::Error> {
        self.server.await
    }
}
use actix_web::{web, App, HttpServer};
use templates::Templates;

mod templates;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let templates = web::Data::new(Templates::new());

    HttpServer::new(move || {
        App::new()
            .app_data(templates.clone())
            .configure(routes::configure)
    })
    .bind("localhost:3000")?
    .run()
    .await
}
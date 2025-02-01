use actix_web::{App, web, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    url: String
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>URL Shortener</title>
            </head>
            <body>
                <h1>URL Shortener</h1>
                <form method="post" action="/submit">
                    <input type="text" name="url" placeholder="Enter URL" required>
                    <button type="submit">Shorten</button>
                </form>
            </body>
            </html>
            "#
        )
}

async fn handle_submit(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Recieved URL: {}", form.url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(handle_submit))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
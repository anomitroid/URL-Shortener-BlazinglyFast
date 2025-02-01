use actix_web::{HttpResponse, get, web};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAssets;

#[get("/static/{file:.*}")]
pub async fn serve_static(file: web::Path<String>) -> HttpResponse {
    let filename = file.into_inner();
    if let Some(content) = StaticAssets::get(&filename) {
        HttpResponse::Ok()
            .content_type(mime_guess::from_path(&filename).first_or_octet_stream())
            .body(content.data.into_owned())
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}
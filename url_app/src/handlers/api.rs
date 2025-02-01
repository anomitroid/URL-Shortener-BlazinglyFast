use actix_web::{web, HttpResponse};
use crate::{AppState, models::UrlEntry};
use url::Url;
use nanoid::nanoid;

#[derive(serde::Deserialize)]
pub struct ShortenRequest {
    url: String,
}

pub async fn shorten_url(
    form: web::Form<ShortenRequest>,
    data: web::Data<AppState>,
) -> HttpResponse {
    // Validate URL
    let url = match Url::parse(&form.url) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid URL format"),
    };

    // Generate short ID
    let short_id = nanoid!(8);
    let created_at = chrono::Utc::now();

    // Store entry
    data.store.insert(short_id.clone(), UrlEntry {
        original_url: url.to_string(),
        short_id: short_id.clone(),
        clicks: 0,
        created_at,
    });

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}
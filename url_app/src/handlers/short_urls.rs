use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::{AppState, UrlEntry};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub full_url: String,
}

pub async fn create_short_url(
    form: web::Form<FormData>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let short_id = Uuid::new_v4().to_string()[..8].to_string();
    
    data.store.insert(short_id.clone(), UrlEntry {
        full_url: form.full_url.clone(),
        short_id: short_id.clone(),
        clicks: 0,
    });

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}
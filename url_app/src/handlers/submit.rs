use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub url: String,
}

pub async fn handle_submit(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Received URL: {}", form.url))
}
use actix_web::{web, HttpResponse};
use crate::templates::Templates;

pub async fn index(templates: web::Data<Templates>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(templates.index.clone())
}
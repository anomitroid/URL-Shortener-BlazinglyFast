use actix_web::web;
use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(handlers::index::index))
        .route("/short_urls", web::post().to(handlers::short_urls::create_short_url))
        .route("/{short_id}", web::get().to(handlers::redirect::redirect));
}
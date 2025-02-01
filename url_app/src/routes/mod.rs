use actix_web::web;
use crate::handlers;
use actix_files::Files;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(Files::new("/static", "./static").show_files_listing())
        .route("/", web::get().to(handlers::frontend::index_handler))
        .route("/shorten", web::post().to(handlers::api::shorten_url))
        .route("/{id}", web::get().to(handlers::redirect::redirect_handler));
}
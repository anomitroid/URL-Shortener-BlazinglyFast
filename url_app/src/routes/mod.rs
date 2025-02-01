use actix_web::web;
use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(handlers::static_files::serve_static)
        .route("/", web::get().to(handlers::frontend::index_handler))
        .route("/shorten", web::post().to(handlers::api::shorten_url))
        .route("/{id}", web::get().to(handlers::redirect::redirect_handler));
}
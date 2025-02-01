use actix_web::web;
use crate::handlers::{index, submit};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index::index))
        .route("/submit", web::post().to(submit::handle_submit));
}
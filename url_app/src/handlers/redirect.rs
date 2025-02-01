use actix_web::{web, HttpResponse};
use crate::AppState;

pub async fn redirect_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let short_id = path.into_inner();

    match data.store.get_mut(&short_id) {
        Some(mut entry) => {
            entry.clicks += 1;
            HttpResponse::TemporaryRedirect()
                .append_header(("Location", entry.original_url.clone()))
                .finish()
        }
        None => HttpResponse::NotFound()
            .content_type("text/html")
            .body("<h1>404 - URL Not Found</h1>"),
    }
}
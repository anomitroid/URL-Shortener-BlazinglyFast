use actix_web::{web, HttpResponse};
use crate::AppState;

pub async fn redirect(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let short_id = path.into_inner();
    
    if let Some(mut entry) = data.store.get_mut(&short_id) {
        entry.clicks += 1;
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", entry.full_url.clone()))
            .finish();
    }

    HttpResponse::NotFound().body("URL not found")
}
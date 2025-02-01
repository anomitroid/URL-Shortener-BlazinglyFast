use actix_web::{web, HttpResponse, Responder};
use crate::AppState;

pub async fn index_handler(data: web::Data<AppState>) -> impl Responder {
    let mut rows = String::new();
    let store = &data.store;

    for entry in store.iter() {
        rows.push_str(&format!(
            r#"<tr>
                <td><a href="{}" target="_blank">{}</a></td>
                <td><a href="/{}" class="short-url">/{}</a></td>
                <td>{}</td>
                <td>{}</td>
            </tr>"#,
            entry.original_url,
            entry.original_url,
            entry.short_id,
            entry.short_id,
            entry.clicks,
            entry.created_at.format("%Y-%m-%d %H:%M:%S")
        ));
    }

    let html = (*crate::handlers::INDEX_HTML)
        .replace("{ROWS}", &rows)
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}
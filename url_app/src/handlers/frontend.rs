use actix_web::{web, HttpResponse, Result};
use crate::AppState;

pub async fn index_handler(data: web::Data<AppState>) -> Result<HttpResponse> {
    let urls = sqlx::query_as!(
        crate::models::UrlEntry,
        r#"
        SELECT id, original_url, short_id, clicks, created_at
        FROM urls
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|_| crate::errors::AppError::InternalError)?;

    let mut rows = String::new();
    
    for entry in urls {
        rows.push_str(&format!(
            r#"<tr>
                <td class="original-url"><a href="{0}" target="_blank" title="{0}">{1}</a></td>
                <td><a href="/{2}" class="short-url" title="Click to copy" target="_blank">{2}</a></td>
                <td class="text-center">{3}</td>
                <td>{4}</td>
            </tr>"#,
            entry.original_url,
            if entry.original_url.len() > 50 {
                format!("{}...", &entry.original_url[..50])
            } else {
                entry.original_url.clone()
            },
            entry.short_id,
            entry.clicks,
            entry.created_at.format("%b %d, %Y at %H:%M")
        ));
    }

    let html = (*crate::handlers::INDEX_HTML)
        .replace("{ROWS}", &rows)
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"));

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}
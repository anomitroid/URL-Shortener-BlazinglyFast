// url_app/src/handlers/frontend.rs

use actix_web::{web, HttpResponse, Result};
use crate::AppState;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as base64;
use log;

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
}

pub async fn index_handler(
    data: web::Data<AppState>,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, crate::errors::AppError> {
    const ITEMS_PER_PAGE: i64 = 8;
    let current_page = params.page.unwrap_or(1).max(1);
    
    // Get total count
    let total_urls: i64 = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "total_urls!" FROM urls"#
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        log::error!("Count query error: {}", e);
        crate::errors::AppError::InternalError
    })?;

    let total_pages = (total_urls as f64 / ITEMS_PER_PAGE as f64).ceil() as i64;
    let current_page = current_page.min(total_pages).max(1);
    let offset = (current_page - 1) * ITEMS_PER_PAGE;

    // Get paginated results
    let urls = sqlx::query_as!(
        crate::models::UrlEntry,
        r#"
        SELECT id, original_url, short_id, clicks, created_at, qr_code
        FROM urls
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        ITEMS_PER_PAGE,
        offset
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        log::error!("Database error: {}", e);
        crate::errors::AppError::InternalError
    })?;

    let mut rows = String::new();
    
    for entry in urls {
        let qr_code_base64 = base64.encode(&entry.qr_code);
        let qr_img = format!("data:image/svg+xml;base64,{}", qr_code_base64);

        rows.push_str(&format!(
            r#"<tr>
                <td class="original-url"><a href="{0}" target="_blank" title="{0}">{1}</a></td>
                <td><a href="/{2}" class="short-url" title="Click to copy" target="_blank">{2}</a></td>
                <td><img src="{5}" class="qr-code" onclick="enlargeQR(this)" style="width: 50px; cursor: pointer;"/></td>
                <td class="text-center clicks">{3}</td>
                <td class="createdTime">{4}</td>
            </tr>"#,
            entry.original_url,
            if entry.original_url.len() > 50 {
                format!("{}...", &entry.original_url[..50])
            } else {
                entry.original_url.clone()
            },
            entry.short_id,
            entry.clicks,
            entry.created_at.format("%b %d, %Y at %H:%M"),
            qr_img
        ));
    }

    // Build pagination controls
    let mut pagination = String::new();
    if total_pages > 1 {
        pagination.push_str(r#"<nav aria-label="Page navigation"><ul class="pagination justify-content-center">"#);
        
        // Previous button
        if current_page > 1 {
            pagination.push_str(&format!(
                r#"<li class="page-item"><a class="page-link" href="?page={}">&laquo;</a></li>"#,
                current_page - 1
            ));
        }

        // Page numbers
        for page in 1..=total_pages {
            let active = if page == current_page { " active" } else { "" };
            pagination.push_str(&format!(
                r#"<li class="page-item{}"><a class="page-link" href="?page={}">{}</a></li>"#,
                active, page, page
            ));
        }

        // Next button
        if current_page < total_pages {
            pagination.push_str(&format!(
                r#"<li class="page-item"><a class="page-link" href="?page={}">&raquo;</a></li>"#,
                current_page + 1
            ));
        }

        pagination.push_str("</ul></nav>");
    }

    let html = (*crate::handlers::INDEX_HTML)
        .replace("{ROWS}", &rows)
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
        .replace("{PAGINATION}", &pagination);

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}
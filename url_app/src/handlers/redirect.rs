use actix_web::{web, HttpResponse};
use crate::AppState;

pub async fn redirect_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, crate::errors::AppError> {
    let short_id = path.into_inner();

    let url = sqlx::query!(
        r#"
        UPDATE urls
        SET clicks = clicks + 1
        WHERE short_id = $1
        RETURNING original_url
        "#,
        short_id
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|_| crate::errors::AppError::InternalError)?;

    match url {
        Some(record) => Ok(HttpResponse::TemporaryRedirect()
            .append_header(("Location", record.original_url))
            .finish()),
        None => Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body("<h1>404 - URL Not Found</h1>")),
    }
}
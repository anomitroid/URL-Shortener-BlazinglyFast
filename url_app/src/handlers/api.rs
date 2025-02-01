use actix_web::{web, HttpResponse};
use crate::AppState;
use url::Url;
use nanoid::nanoid;

#[derive(serde::Deserialize)]
pub struct ShortenRequest {
    url: String,
}

pub async fn shorten_url(
    form: web::Form<ShortenRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, crate::errors::AppError> {
    let url = Url::parse(&form.url)
        .map_err(|_| crate::errors::AppError::InvalidUrl)?;

    let short_id = nanoid!(8);

    sqlx::query!(
        r#"
        INSERT INTO urls (original_url, short_id)
        VALUES ($1, $2)
        "#,
        url.to_string(),
        short_id
    )
    .execute(&data.db)
    .await
    .map_err(|_| crate::errors::AppError::InternalError)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}
use actix_web::{web, HttpResponse};
use crate::AppState;
use url::Url;
use nanoid::nanoid;
use dotenvy::var;
use qrcode_generator::QrCodeEcc;

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
    let base_url = var("BASE_URL").expect("BASE_URL must be set");
    let short_url = format!("{}/{}", base_url, short_id);

    let qr_code = qrcode_generator::to_svg_to_string(short_url, QrCodeEcc::Low, 200, None::<&str>)
        .map_err(|_| crate::errors::AppError::InternalError)?;

    let qr_code = if qr_code.is_empty() {
        return Err(crate::errors::AppError::InternalError);
    } else {
        qr_code
    };

    sqlx::query!(
        r#"
        INSERT INTO urls (original_url, short_id, qr_code)
        VALUES ($1, $2, $3)
        "#,
        url.to_string(),
        short_id,
        qr_code
    )
    .execute(&data.db)
    .await
    .map_err(|_| crate::errors::AppError::InternalError)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}
use actix_web::{web, HttpResponse, HttpRequest};
use crate::AppState;
use url::Url;
use nanoid::nanoid;
use dotenvy::var;
use qrcode_generator::QrCodeEcc;
use redis::AsyncCommands;

#[derive(serde::Deserialize)]
pub struct ShortenRequest {
    url: String,
}

pub async fn shorten_url(
    req: HttpRequest,
    form: web::Form<ShortenRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, crate::errors::AppError> {
    const RATE_LIMIT: u64 = 20;
    const RATE_LIMIT_WINDOW: u64 = 60;

    let client_ip = req.peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".into());

    let key = format!("rate_limit:{}", client_ip);

    let mut conn = data.redis.get().await.map_err(|e| {
        log::error!("Redis connection error: {}", e);
        crate::errors::AppError::InternalError
    })?;

    // Use INCR command
    let count: u64 = conn.incr(&key, 1).await.map_err(|e| {
        log::error!("Redis error: {}", e);
        crate::errors::AppError::InternalError
    })?;

    // Set expiration only on first increment
    if count == 1 {
        conn.expire(&key, RATE_LIMIT_WINDOW as i64).await.map_err(|e| {
            log::error!("Redis error: {}", e);
            crate::errors::AppError::InternalError
        })?;
    }

    if count > RATE_LIMIT {
        return Err(crate::errors::AppError::TooManyRequests);
    }

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
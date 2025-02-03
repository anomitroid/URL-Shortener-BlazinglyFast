use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid URL format")]
    InvalidUrl,
    #[error("Short URL not found")]
    NotFound,
    #[error("Internal server error")]
    InternalError,
    #[error("Too many requests. Please try again later.")]
    TooManyRequests,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InvalidUrl => HttpResponse::BadRequest().body(self.to_string()),
            AppError::NotFound => HttpResponse::NotFound().body(self.to_string()),
            AppError::InternalError => HttpResponse::InternalServerError().body(self.to_string()),
            AppError::TooManyRequests => HttpResponse::TooManyRequests().body(self.to_string()),
        }
    }
}
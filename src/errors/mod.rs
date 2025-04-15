use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Invalid request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Unauthorized: {0}")]
    #[allow(dead_code)] // Will be used when authentication is implemented
    Unauthorized(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error = self.to_string();
        let response = ErrorResponse { error };
        
        match self {
            AppError::NotFound(_) => HttpResponse::NotFound().json(response),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(response),
            AppError::InternalError(_) => {
                tracing::error!("Internal error: {}", self);
                HttpResponse::InternalServerError().json(response)
            }
            AppError::Unauthorized(_) => HttpResponse::Unauthorized().json(response),
        }
    }
} 
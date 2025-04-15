use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateTelemetryRequest, TelemetryQuery};
use crate::services::TelemetryService;

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

/// Delete request payload
#[derive(Deserialize)]
pub struct DeleteOldRecordsRequest {
    older_than: DateTime<Utc>,
}

/// Response for successful record creation
#[derive(Serialize)]
struct CreateResponse {
    id: Uuid,
}

/// Response for successful record deletion
#[derive(Serialize)]
struct DeleteResponse {
    deleted_count: usize,
}

/// Health check handler
pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    HttpResponse::Ok().json(response)
}

/// Create a new telemetry record
pub async fn create_telemetry(
    service: web::Data<TelemetryService>,
    payload: web::Json<CreateTelemetryRequest>,
) -> Result<HttpResponse, AppError> {
    let id = service
        .create_telemetry(payload.into_inner())
        .await?;
        
    let response = CreateResponse { id };
    Ok(HttpResponse::Created().json(response))
}

/// Get telemetry data by ID
pub async fn get_telemetry_by_id(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let id = Uuid::parse_str(&id).map_err(|_| {
        AppError::BadRequest("Invalid UUID format".to_string())
    })?;
    
    let telemetry = service.get_telemetry_by_id(id).await?;
    Ok(HttpResponse::Ok().json(telemetry))
}

/// Get telemetry data for a specific device
pub async fn get_device_telemetry(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
    query: web::Query<TelemetryQuery>,
) -> Result<HttpResponse, AppError> {
    let device_id = path.into_inner();
    let telemetry = service
        .get_device_telemetry(
            &device_id,
            query.start_time,
            query.end_time,
            query.limit,
        )
        .await?;
        
    Ok(HttpResponse::Ok().json(telemetry))
}

/// Delete telemetry records older than a specific timestamp
pub async fn delete_old_records(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
    payload: web::Json<DeleteOldRecordsRequest>,
) -> Result<HttpResponse, AppError> {
    let device_id = path.into_inner();
    let count = service
        .delete_old_records(&device_id, payload.older_than)
        .await?;
        
    let response = DeleteResponse {
        deleted_count: count,
    };
    
    Ok(HttpResponse::Ok().json(response))
} 
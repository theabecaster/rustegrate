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

/// Tracks list response
#[derive(Serialize)]
struct TracksResponse {
    tracks: Vec<String>,
}

/// Cars list response
#[derive(Serialize)]
struct CarsResponse {
    cars: Vec<String>,
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
    let id = service.create_telemetry(payload.into_inner()).await?;

    let response = CreateResponse { id };
    Ok(HttpResponse::Created().json(response))
}

/// Get telemetry data by ID
pub async fn get_telemetry_by_id(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;

    let telemetry = service.get_telemetry_by_id(id).await?;
    Ok(HttpResponse::Ok().json(telemetry))
}

/// Get telemetry data for a specific driver
pub async fn get_driver_telemetry(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
    query: web::Query<TelemetryQuery>,
) -> Result<HttpResponse, AppError> {
    let driver_id = path.into_inner();
    
    // Extract filters from the query
    let track_name = query.track_name.as_deref();
    let car_name = query.car_name.as_deref();
    let session_type = query.session_type.as_deref();
    
    let telemetry = service
        .get_driver_telemetry(
            &driver_id, 
            query.start_time, 
            query.end_time, 
            track_name,
            car_name,
            session_type,
            query.limit
        )
        .await?;

    Ok(HttpResponse::Ok().json(telemetry))
}

/// Get telemetry data for a specific session
pub async fn get_session_telemetry(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
    query: web::Query<LimitQuery>,
) -> Result<HttpResponse, AppError> {
    let session_id = path.into_inner();
    let telemetry = service
        .get_session_telemetry(&session_id, query.limit.unwrap_or(100))
        .await?;

    Ok(HttpResponse::Ok().json(telemetry))
}

/// Query for endpoints that only need a limit
#[derive(Deserialize)]
pub struct LimitQuery {
    limit: Option<usize>,
}

/// Get all tracks in the system
pub async fn get_tracks(
    service: web::Data<TelemetryService>,
) -> Result<HttpResponse, AppError> {
    let tracks = service.get_tracks().await?;
    let response = TracksResponse { tracks };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Get all cars in the system
pub async fn get_cars(
    service: web::Data<TelemetryService>,
) -> Result<HttpResponse, AppError> {
    let cars = service.get_cars().await?;
    let response = CarsResponse { cars };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Delete telemetry records older than a specific timestamp
pub async fn delete_old_records(
    service: web::Data<TelemetryService>,
    path: web::Path<String>,
    payload: web::Json<DeleteOldRecordsRequest>,
) -> Result<HttpResponse, AppError> {
    let driver_id = path.into_inner();
    let count = service
        .delete_old_records(&driver_id, payload.older_than)
        .await?;

    let response = DeleteResponse {
        deleted_count: count,
    };

    Ok(HttpResponse::Ok().json(response))
}

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateTelemetryRequest, TelemetryData};
use crate::storage::TelemetryStore;

/// Service for handling telemetry operations
pub struct TelemetryService {
    store: TelemetryStore,
}

impl TelemetryService {
    /// Create a new telemetry service with the provided store
    pub fn new(store: TelemetryStore) -> Self {
        Self { store }
    }

    /// Create a new telemetry record
    pub async fn create_telemetry(
        &self,
        request: CreateTelemetryRequest,
    ) -> Result<Uuid, AppError> {
        let telemetry = TelemetryData::from(request);
        let id = self
            .store
            .add(telemetry)
            .await
            .map_err(AppError::InternalError)?;

        Ok(id)
    }

    /// Get telemetry data for a specific driver
    pub async fn get_driver_telemetry(
        &self,
        driver_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        track_name: Option<&str>,
        car_name: Option<&str>,
        session_type: Option<&str>,
        limit: usize,
    ) -> Result<Vec<TelemetryData>, AppError> {
        let telemetry = self
            .store
            .get_by_driver(driver_id, start_time, end_time, track_name, car_name, session_type, limit)
            .await;
        Ok(telemetry)
    }
    
    /// Get telemetry data for a specific session
    pub async fn get_session_telemetry(
        &self,
        session_id: &str,
        limit: usize,
    ) -> Result<Vec<TelemetryData>, AppError> {
        let telemetry = self
            .store
            .get_by_session(session_id, limit)
            .await;
        Ok(telemetry)
    }

    /// Get a specific telemetry record by ID
    pub async fn get_telemetry_by_id(&self, id: Uuid) -> Result<TelemetryData, AppError> {
        self.store
            .get_by_id(id)
            .await
            .ok_or_else(|| AppError::NotFound(format!("Telemetry with ID {} not found", id)))
    }

    /// Delete old telemetry records for a driver
    pub async fn delete_old_records(
        &self,
        driver_id: &str,
        older_than: DateTime<Utc>,
    ) -> Result<usize, AppError> {
        let count = self.store.delete_old_records(driver_id, older_than).await;
        Ok(count)
    }
    
    /// Get all unique tracks
    pub async fn get_tracks(&self) -> Result<Vec<String>, AppError> {
        let tracks = self.store.get_tracks().await;
        Ok(tracks)
    }
    
    /// Get all unique cars
    pub async fn get_cars(&self) -> Result<Vec<String>, AppError> {
        let cars = self.store.get_cars().await;
        Ok(cars)
    }
}

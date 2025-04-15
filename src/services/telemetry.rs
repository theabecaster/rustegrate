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
            .map_err(|e| AppError::InternalError(e))?;
            
        Ok(id)
    }
    
    /// Get telemetry data for a specific device
    pub async fn get_device_telemetry(
        &self,
        device_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: usize,
    ) -> Result<Vec<TelemetryData>, AppError> {
        let telemetry = self.store.get_by_device(device_id, start_time, end_time, limit).await;
        Ok(telemetry)
    }
    
    /// Get a specific telemetry record by ID
    pub async fn get_telemetry_by_id(&self, id: Uuid) -> Result<TelemetryData, AppError> {
        self.store
            .get_by_id(id)
            .await
            .ok_or_else(|| AppError::NotFound(format!("Telemetry with ID {} not found", id)))
    }
    
    /// Delete old telemetry records for a device
    pub async fn delete_old_records(
        &self,
        device_id: &str,
        older_than: DateTime<Utc>,
    ) -> Result<usize, AppError> {
        let count = self.store.delete_old_records(device_id, older_than).await;
        Ok(count)
    }
} 
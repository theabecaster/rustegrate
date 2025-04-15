use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents telemetry data received from a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    /// Unique identifier for the telemetry record
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    
    /// Identifier of the device that sent the telemetry
    pub device_id: String,
    
    /// Temperature reading from the device (in Celsius)
    pub temperature: f32,
    
    /// Optional humidity reading (in percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<f32>,
    
    /// Optional pressure reading (in hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure: Option<f32>,
    
    /// Timestamp when the telemetry was recorded
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
}

/// Represents a request to create a new telemetry record
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTelemetryRequest {
    pub device_id: String,
    pub temperature: f32,
    pub humidity: Option<f32>,
    pub pressure: Option<f32>,
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
}

impl From<CreateTelemetryRequest> for TelemetryData {
    fn from(req: CreateTelemetryRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            device_id: req.device_id,
            temperature: req.temperature,
            humidity: req.humidity,
            pressure: req.pressure,
            timestamp: req.timestamp,
        }
    }
}

/// Query parameters for retrieving telemetry data
#[derive(Debug, Deserialize)]
pub struct TelemetryQuery {
    /// Optional start time filter (inclusive)
    pub start_time: Option<DateTime<Utc>>,
    
    /// Optional end time filter (inclusive)
    pub end_time: Option<DateTime<Utc>>,
    
    /// Maximum number of records to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    100
} 
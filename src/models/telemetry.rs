use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents telemetry data received from iRacing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    /// Unique identifier for the telemetry record
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,

    /// Identifier of the session/driver
    pub driver_id: String,

    /// Session information
    pub session_id: String,
    pub session_type: String, // Practice, Qualifying, Race
    pub track_name: String,
    pub car_name: String,

    /// Vehicle performance metrics
    pub speed: f32,            // Speed in km/h
    pub rpm: f32,              // Engine RPM
    pub gear: i8,              // Current gear (-1 for reverse, 0 for neutral, 1-8 for gears)
    
    /// Lap information
    pub current_lap: u32,
    pub last_lap_time: Option<f32>,  // Last lap time in seconds
    pub best_lap_time: Option<f32>,  // Best lap time in seconds
    
    /// Vehicle status
    pub fuel_level: f32,       // Fuel remaining (percentage or liters)
    pub throttle_position: f32, // 0.0 to 1.0
    pub brake_position: f32,   // 0.0 to 1.0
    pub clutch_position: f32,  // 0.0 to 1.0
    pub steering_angle: f32,   // Steering angle in degrees
    
    /// Tire data
    pub tire_temps_fl: f32,    // Front left tire temperature
    pub tire_temps_fr: f32,    // Front right tire temperature
    pub tire_temps_rl: f32,    // Rear left tire temperature
    pub tire_temps_rr: f32,    // Rear right tire temperature
    
    /// Forces
    pub lateral_g: f32,        // Lateral G-force
    pub longitudinal_g: f32,   // Longitudinal G-force
    pub vertical_g: f32,       // Vertical G-force
    
    /// Position data
    pub track_position: f32,   // Position on track (0.0 to 1.0 representing lap completion)
    pub position_x: f32,       // X coordinate
    pub position_y: f32,       // Y coordinate
    pub position_z: f32,       // Z coordinate
    
    /// Timestamp when the telemetry was recorded
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
}

/// Represents a request to create a new telemetry record
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTelemetryRequest {
    pub driver_id: String,
    pub session_id: String,
    pub session_type: String,
    pub track_name: String,
    pub car_name: String,
    pub speed: f32,
    pub rpm: f32,
    pub gear: i8,
    pub current_lap: u32,
    pub last_lap_time: Option<f32>,
    pub best_lap_time: Option<f32>,
    pub fuel_level: f32,
    pub throttle_position: f32,
    pub brake_position: f32,
    pub clutch_position: f32,
    pub steering_angle: f32,
    pub tire_temps_fl: f32,
    pub tire_temps_fr: f32,
    pub tire_temps_rl: f32,
    pub tire_temps_rr: f32,
    pub lateral_g: f32,
    pub longitudinal_g: f32,
    pub vertical_g: f32,
    pub track_position: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
}

impl From<CreateTelemetryRequest> for TelemetryData {
    fn from(req: CreateTelemetryRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            driver_id: req.driver_id,
            session_id: req.session_id,
            session_type: req.session_type,
            track_name: req.track_name,
            car_name: req.car_name,
            speed: req.speed,
            rpm: req.rpm,
            gear: req.gear,
            current_lap: req.current_lap,
            last_lap_time: req.last_lap_time,
            best_lap_time: req.best_lap_time,
            fuel_level: req.fuel_level,
            throttle_position: req.throttle_position,
            brake_position: req.brake_position,
            clutch_position: req.clutch_position,
            steering_angle: req.steering_angle,
            tire_temps_fl: req.tire_temps_fl,
            tire_temps_fr: req.tire_temps_fr,
            tire_temps_rl: req.tire_temps_rl,
            tire_temps_rr: req.tire_temps_rr,
            lateral_g: req.lateral_g,
            longitudinal_g: req.longitudinal_g,
            vertical_g: req.vertical_g,
            track_position: req.track_position,
            position_x: req.position_x,
            position_y: req.position_y,
            position_z: req.position_z,
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

    /// Filter by track
    pub track_name: Option<String>,
    
    /// Filter by car
    pub car_name: Option<String>,
    
    /// Filter by session type
    pub session_type: Option<String>,
    
    /// Maximum number of records to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    100
}

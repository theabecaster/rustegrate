use chrono::{DateTime, Utc};
use dashmap::DashMap;
use uuid::Uuid;

use crate::models::TelemetryData;

/// In-memory telemetry data store using DashMap for concurrent access
pub struct TelemetryStore {
    /// Maps device_id to a vector of telemetry records
    data: DashMap<String, Vec<TelemetryData>>,
}

impl TelemetryStore {
    /// Create a new telemetry store
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }
    
    /// Add a telemetry record to the store
    pub async fn add(&self, telemetry: TelemetryData) -> Result<Uuid, String> {
        let device_id = telemetry.device_id.clone();
        let id = telemetry.id;
        
        // Insert into the device's telemetry list, creating it if it doesn't exist
        self.data
            .entry(device_id)
            .or_insert_with(Vec::new)
            .push(telemetry);
            
        Ok(id)
    }
    
    /// Get telemetry data for a specific device, optionally filtered by time range
    pub async fn get_by_device(
        &self,
        device_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: usize,
    ) -> Vec<TelemetryData> {
        match self.data.get(device_id) {
            Some(data) => {
                let filtered = data
                    .iter()
                    .filter(|t| {
                        // Apply time range filters if provided
                        let after_start = start_time
                            .map(|st| t.timestamp >= st)
                            .unwrap_or(true);
                            
                        let before_end = end_time
                            .map(|et| t.timestamp <= et)
                            .unwrap_or(true);
                            
                        after_start && before_end
                    })
                    .take(limit)
                    .cloned()
                    .collect();
                    
                filtered
            }
            None => Vec::new(),
        }
    }
    
    /// Delete telemetry records for a device older than the specified timestamp
    pub async fn delete_old_records(
        &self,
        device_id: &str,
        older_than: DateTime<Utc>,
    ) -> usize {
        if let Some(mut data) = self.data.get_mut(device_id) {
            let initial_count = data.len();
            data.retain(|t| t.timestamp >= older_than);
            let removed = initial_count - data.len();
            
            removed
        } else {
            0
        }
    }
    
    /// Get telemetry data by its unique ID
    pub async fn get_by_id(&self, id: Uuid) -> Option<TelemetryData> {
        for device_data in self.data.iter() {
            if let Some(telemetry) = device_data.iter().find(|t| t.id == id) {
                return Some(telemetry.clone());
            }
        }
        
        None
    }
} 
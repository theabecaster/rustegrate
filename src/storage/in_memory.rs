use chrono::{DateTime, Utc};
use dashmap::DashMap;
use uuid::Uuid;

use crate::models::TelemetryData;

/// In-memory telemetry data store using DashMap for concurrent access
pub struct TelemetryStore {
    /// Maps driver_id to a vector of telemetry records
    data: DashMap<String, Vec<TelemetryData>>,
    /// Maps session_id to a vector of telemetry records for quick session lookup
    sessions: DashMap<String, Vec<Uuid>>,
}

impl Default for TelemetryStore {
    fn default() -> Self {
        Self {
            data: DashMap::new(),
            sessions: DashMap::new(),
        }
    }
}

impl TelemetryStore {
    /// Create a new telemetry store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a telemetry record to the store
    pub async fn add(&self, telemetry: TelemetryData) -> Result<Uuid, String> {
        let driver_id = telemetry.driver_id.clone();
        let session_id = telemetry.session_id.clone();
        let id = telemetry.id;

        // Insert into the driver's telemetry list, creating it if it doesn't exist
        self.data.entry(driver_id).or_default().push(telemetry.clone());
        
        // Add reference to session mapping
        self.sessions.entry(session_id).or_default().push(id);

        Ok(id)
    }

    /// Get telemetry data for a specific driver, optionally filtered by time range and other criteria
    pub async fn get_by_driver(
        &self,
        driver_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        track_name: Option<&str>,
        car_name: Option<&str>,
        session_type: Option<&str>,
        limit: usize,
    ) -> Vec<TelemetryData> {
        match self.data.get(driver_id) {
            Some(data) => {
                let filtered = data
                    .iter()
                    .filter(|t| {
                        // Apply time range filters if provided
                        let after_start = start_time.map(|st| t.timestamp >= st).unwrap_or(true);
                        let before_end = end_time.map(|et| t.timestamp <= et).unwrap_or(true);
                        
                        // Apply track filter if provided
                        let track_match = track_name
                            .map(|track| t.track_name == track)
                            .unwrap_or(true);
                            
                        // Apply car filter if provided
                        let car_match = car_name
                            .map(|car| t.car_name == car)
                            .unwrap_or(true);
                            
                        // Apply session type filter if provided
                        let session_match = session_type
                            .map(|session| t.session_type == session)
                            .unwrap_or(true);

                        after_start && before_end && track_match && car_match && session_match
                    })
                    .take(limit)
                    .cloned()
                    .collect();

                filtered
            }
            None => Vec::new(),
        }
    }
    
    /// Get telemetry data for a specific session
    pub async fn get_by_session(
        &self,
        session_id: &str,
        limit: usize,
    ) -> Vec<TelemetryData> {
        match self.sessions.get(session_id) {
            Some(ids) => {
                let mut results = Vec::new();
                let mut count = 0;
                
                // Get each record by UUID
                for id in ids.iter() {
                    if count >= limit {
                        break;
                    }
                    
                    if let Some(telemetry) = self.get_by_id(*id).await {
                        results.push(telemetry);
                        count += 1;
                    }
                }
                
                results
            },
            None => Vec::new(),
        }
    }

    /// Delete telemetry records for a driver older than the specified timestamp
    pub async fn delete_old_records(&self, driver_id: &str, older_than: DateTime<Utc>) -> usize {
        if let Some(mut data) = self.data.get_mut(driver_id) {
            let initial_count = data.len();
            
            // Get sessions and IDs to remove
            let sessions_to_update: Vec<(String, Uuid)> = data
                .iter()
                .filter(|t| t.timestamp < older_than)
                .map(|t| (t.session_id.clone(), t.id))
                .collect();
                
            // Keep only records newer than the cutoff
            data.retain(|t| t.timestamp >= older_than);
            
            // Update session mappings
            for (session_id, id) in sessions_to_update {
                if let Some(mut ids) = self.sessions.get_mut(&session_id) {
                    ids.retain(|&stored_id| stored_id != id);
                }
            }

            initial_count - data.len()
        } else {
            0
        }
    }

    /// Get telemetry data by its unique ID
    pub async fn get_by_id(&self, id: Uuid) -> Option<TelemetryData> {
        for driver_data in self.data.iter() {
            if let Some(telemetry) = driver_data.iter().find(|t| t.id == id) {
                return Some(telemetry.clone());
            }
        }

        None
    }
    
    /// Get all unique tracks in the dataset
    pub async fn get_tracks(&self) -> Vec<String> {
        let mut tracks = std::collections::HashSet::new();
        
        for driver_data in self.data.iter() {
            for telemetry in driver_data.iter() {
                tracks.insert(telemetry.track_name.clone());
            }
        }
        
        tracks.into_iter().collect()
    }
    
    /// Get all unique cars in the dataset
    pub async fn get_cars(&self) -> Vec<String> {
        let mut cars = std::collections::HashSet::new();
        
        for driver_data in self.data.iter() {
            for telemetry in driver_data.iter() {
                cars.insert(telemetry.car_name.clone());
            }
        }
        
        cars.into_iter().collect()
    }
}

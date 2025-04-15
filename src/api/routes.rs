use actix_web::web;

use super::handlers;

/// Configure the API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Telemetry endpoints
            .service(
                web::scope("/telemetry")
                    // POST /api/v1/telemetry - Create a new telemetry record
                    .route("", web::post().to(handlers::create_telemetry))
                    // GET /api/v1/telemetry/{id} - Get a specific telemetry record
                    .route("/{id}", web::get().to(handlers::get_telemetry_by_id)),
            )
            // Device endpoints
            .service(
                web::scope("/devices/{device_id}")
                    // GET /api/v1/devices/{device_id}/telemetry - Get telemetry for a device
                    .route("/telemetry", web::get().to(handlers::get_device_telemetry))
                    // DELETE /api/v1/devices/{device_id}/telemetry - Delete old telemetry records
                    .route("/telemetry", web::delete().to(handlers::delete_old_records)),
            )
            // Health check endpoint
            .route("/health", web::get().to(handlers::health_check)),
    );
}

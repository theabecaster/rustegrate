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
            // Driver endpoints
            .service(
                web::scope("/drivers/{driver_id}")
                    // GET /api/v1/drivers/{driver_id}/telemetry - Get telemetry for a driver
                    .route("/telemetry", web::get().to(handlers::get_driver_telemetry))
                    // DELETE /api/v1/drivers/{driver_id}/telemetry - Delete old telemetry records
                    .route("/telemetry", web::delete().to(handlers::delete_old_records)),
            )
            // Session endpoints
            .service(
                web::scope("/sessions")
                    // GET /api/v1/sessions/{session_id}/telemetry - Get telemetry for a session
                    .route("/{session_id}/telemetry", web::get().to(handlers::get_session_telemetry)),
            )
            // Racing metadata endpoints
            .service(
                web::scope("/racing")
                    // GET /api/v1/racing/tracks - Get all available tracks
                    .route("/tracks", web::get().to(handlers::get_tracks))
                    // GET /api/v1/racing/cars - Get all available cars
                    .route("/cars", web::get().to(handlers::get_cars)),
            )
            // Health check endpoint
            .route("/health", web::get().to(handlers::health_check)),
    );
}

mod api;
mod config;
mod errors;
mod models;
mod services;
mod storage;

use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::api::routes;
use crate::config::AppConfig;
use crate::services::TelemetryService;
use crate::storage::TelemetryStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");
    
    // Extract values needed outside the closure
    let host = config.host.clone();
    let port = config.port;
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(config.log_level.clone())
        .init();
    
    // Initialize telemetry store
    let telemetry_store = TelemetryStore::new();
    
    // Create telemetry service
    let telemetry_service = TelemetryService::new(telemetry_store);
    let service_data = web::Data::new(telemetry_service);
    
    // Start HTTP server
    tracing::info!("Starting server at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(service_data.clone())
            .app_data(web::Data::new(config.clone()))
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

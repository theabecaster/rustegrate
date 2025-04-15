use actix_web::{test, web, App};
use chrono::Utc;
use rustegrate::api::routes;
use rustegrate::models::CreateTelemetryRequest;
use rustegrate::services::TelemetryService;
use rustegrate::storage::TelemetryStore;
use serde_json::json;

#[actix_web::test]
async fn test_create_telemetry() {
    // Setup
    let store = TelemetryStore::new();
    let service = TelemetryService::new(store);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(routes::configure),
    )
    .await;

    // Create test payload
    let payload = CreateTelemetryRequest {
        device_id: "test-device-001".to_string(),
        temperature: 23.5,
        humidity: Some(45.0),
        pressure: Some(1013.0),
        timestamp: Utc::now(),
    };

    // Send POST request
    let req = test::TestRequest::post()
        .uri("/api/v1/telemetry")
        .set_json(&payload)
        .to_request();

    // Verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Parse response
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check that the response contains an ID
    assert!(response.get("id").is_some());
}

#[actix_web::test]
async fn test_get_device_telemetry() {
    // Setup
    let store = TelemetryStore::new();

    // Create a test telemetry entry
    let payload = CreateTelemetryRequest {
        device_id: "test-device-002".to_string(),
        temperature: 22.5,
        humidity: Some(40.0),
        pressure: Some(1010.0),
        timestamp: Utc::now(),
    };

    let telemetry = rustegrate::models::TelemetryData::from(payload);
    let _ = store.add(telemetry).await.unwrap();

    let service = TelemetryService::new(store);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(routes::configure),
    )
    .await;

    // Send GET request
    let req = test::TestRequest::get()
        .uri("/api/v1/devices/test-device-002/telemetry")
        .to_request();

    // Verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Parse response
    let body = test::read_body(resp).await;
    let response: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Verify we got at least one telemetry record
    assert!(!response.is_empty());
    assert_eq!(response[0]["device_id"], json!("test-device-002"));
}

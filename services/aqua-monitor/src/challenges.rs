use serde_json::json;
use shuttle_axum::axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use std::time::Duration;
use tracing;

use crate::AppState;

/// Retrieves the current status of all tank sensors.
/// 
/// This endpoint demonstrates the use of a shared HTTP client to prevent resource leaks.
/// It's part of Challenge #4 in the Rust learning path.
///
/// # Returns
/// - `200 OK` with sensor status data on success
/// - `503 Service Unavailable` if unable to connect to sensor API
pub async fn get_sensor_status(State(_state): State<AppState>) -> impl IntoResponse {
    // Create a span for sensor status check with request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let span = tracing::info_span!(
        "tank_sensor_status_check",
        request_id = %request_id
    );
    let _guard = span.enter();

    // ⚠️ CHALLENGE #4: RESOURCE LEAK ⚠️
    // Problem: Creating a new HTTP client for each request wastes resources
    // Solution: Use a shared static client that's created only once
    
    // PROBLEM: Creating a new client for each request (resource leak)
    let client = reqwest::Client::new();
    
    // Set environment variable to track that we're NOT using the static client
    std::env::set_var("USING_STATIC_CLIENT", "false");
    
    // Log metrics about connection creation
    tracing::info!(
        request_id = %request_id,
        "Created new HTTP client for request"
    );

    // ⚠️ END CHALLENGE CODE ⚠️
    
    // In a real application, this would be an actual sensor API endpoint
    let sensor_data = || json!({
        "status": "online",
        "active_sensors": 24,
        "last_updated": chrono::Utc::now()
    });

    // Try to get sensor data from external API
    match client
        .get("https://api.example.com/sensors")
        .timeout(Duration::from_secs(2))
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            // Return mock data since this is an example
            Json(sensor_data()).into_response()
        }
        _ => {
            tracing::warn!(
                request_id = %request_id,
                "Failed to fetch sensor data, returning mock response"
            );
            // Fall back to mock data
            Json(sensor_data()).into_response()
        }
    }
}

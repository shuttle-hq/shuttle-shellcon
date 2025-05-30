use serde_json::json;
use shuttle_axum::axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::time::Duration;
use tracing;

use crate::{AppState, ApiError, TankSettingsSummary, TankReading};

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

/// Retrieves the recent readings for a specific tank.
///
/// This function demonstrates Challenge #1: Async I/O for file operations.
///
/// # Parameters
/// - `tank_id`: The ID of the tank to retrieve readings for
/// - `state`: The application state containing database connections
///
/// # Returns
/// - `200 OK` with readings data on success
/// - Error responses for various failure cases
pub async fn get_tank_readings(
    Path(tank_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Create a span for the entire request timing
    let request_span = tracing::info_span!("tank_readings_request");
    let _request_guard = request_span.enter();

    // Add request ID for correlation and include tank_id in all logs
    let request_id = uuid::Uuid::new_v4().to_string();

    // Start timing the overall request
    let start = std::time::Instant::now();

    tracing::info!(
        request_id = %request_id,
        tank_id = %tank_id,
        operation = "get_tank_readings",
        "Processing tank readings request"
    );

    // ⚠️ CHALLENGE #1: ASYNC I/O ⚠️
    // The aquarium monitoring system needs to read tank configuration settings frequently.
    // Your task: Implement asynchronous file I/O to improve performance.

    // Create a span specifically for file I/O operations
    let io_span = tracing::info_span!("file_io_operation");
    let _io_guard = io_span.enter();

    // Blocking implementation
    let io_start = std::time::Instant::now();

    // Blocking implementation - this blocks the thread
    let config = std::fs::read_to_string("./config/tank_settings.json")
        .unwrap_or_else(|_| "{}".to_string());
    // Simulate additional I/O latency in the blocking implementation
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Parse summarized tank settings
    let settings: TankSettingsSummary = serde_json::from_str(&config).unwrap_or_default();

    let io_duration = io_start.elapsed().as_millis();
    tracing::info!(
        request_id = %request_id,
        tank_id = %tank_id,
        io_duration_ms = io_duration,
        "Tank configuration file I/O completed"
    );
    // ⚠️ END CHALLENGE CODE ⚠️

    // First, check if the tank exists
    if tank_id.is_empty() {
        tracing::warn!("Empty tank ID provided");
        return Err(ApiError::TankNotFound("empty tank ID".to_string()));
    }

    // Async database query
    tracing::debug!("Starting database query");
    let db_start = std::time::Instant::now();
    let readings = sqlx::query_as::<_, TankReading>(
        "SELECT * FROM tank_readings WHERE tank_id = $1 ORDER BY timestamp DESC LIMIT 10",
    )
    .bind(&tank_id)
    .fetch_all(&state.pool)
    .await
    // Propagate database errors
    .map_err(ApiError::Database)?;

    let db_duration = db_start.elapsed().as_millis();
    tracing::debug!(
        request_id = %request_id,
        tank_id = %tank_id,
        db_duration_ms = db_duration,
        db_rows_returned = readings.len(),
        "Tank readings database query completed"
    );

    // If no readings found, you could return a specialized error
    if readings.is_empty() {
        tracing::info!(
            request_id = %request_id,
            tank_id = %tank_id,
            "No readings found for tank"
        );
        return Err(ApiError::TankNotFound(format!("No readings for tank ID: {}", tank_id)));
    }

    // Calculate the total request time
    let total_duration = start.elapsed().as_millis();
    tracing::info!(
        request_id = %request_id,
        tank_id = %tank_id, 
        total_duration_ms = total_duration,
        io_duration_ms = io_duration,
        db_duration_ms = db_duration,
        "Tank readings request completed successfully"
    );

    // Create the response with readings and tank settings
    Ok(Json(json!({
        "tank_id": tank_id,
        "readings": readings,
        "settings": settings,
        "meta": {
            "count": readings.len(),
            "response_time_ms": total_duration
        }
    })))
}

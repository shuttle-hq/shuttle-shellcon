mod challenges;

use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_axum::axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sqlx::{PgPool, Row};
use std::fs;
use tracing;
use thiserror::Error;

// Custom Error Type for aqua-monitor service
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Sensor error: {0}")]
    SensorError(String),

    #[error("Tank not found: {0}")]
    TankNotFound(String),

    #[error("External service error: {0}")]
    ExternalService(#[from] reqwest::Error),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Implement IntoResponse for our custom error type
impl IntoResponse for ApiError {
    fn into_response(self) -> shuttle_axum::axum::response::Response {
        let (status, error_message) = match &self {
            ApiError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            ApiError::SensorError(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Sensor error: {}", msg),
            ),
            ApiError::TankNotFound(id) => {
                (StatusCode::NOT_FOUND, format!("Tank not found: {}", id))
            }
            ApiError::ExternalService(_) => (
                StatusCode::BAD_GATEWAY,
                "External service error".to_string(),
            ),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };

        // Log the error with structured fields
        tracing::error!(
            error.type = std::any::type_name::<Self>(),
            error.message = %error_message,
            error.status = %status.as_u16(),
            "API error occurred"
        );

        // Return status code and JSON error message
        (
            status,
            Json(serde_json::json!({
                "error": error_message,
                "status": status.as_u16(),
                "timestamp": chrono::Utc::now()
            })),
        )
            .into_response()
    }
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct TankReading {
    id: i32,
    tank_id: String,
    temperature: f64,
    ph: f64,
    oxygen_level: f64,
    salinity: f64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct TankSettingsSummary {
    tank_type: Option<String>,
    min_temperature: Option<f64>,
    max_temperature: Option<f64>,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // Initialize database with logging and proper error handling
    tracing::info!("Running database migrations for aqua-monitor...");
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        tracing::error!(error = %e, "Database migration failed for aqua-monitor");
        return Err(anyhow::anyhow!("Database migration failed: {e}").into());
    }
    tracing::info!("Database migrations completed successfully for aqua-monitor.");

    // Initialize state
    let state = AppState { pool };

    // Build router
    let router = Router::new()
        .route("/api/tanks", get(get_all_tanks))
        .route("/api/tanks/:tank_id/readings", get(get_tank_readings))
        .route(
            "/api/challenges/1/validate",
            get(validate_challenge_solution),
        ) // Challenge #1: Async I/O
        .route(
            "/api/challenges/4/validate",
            get(validate_resource_leak_solution),
        ) // Challenge #4: Resource Leak
        .route("/api/sensors/status", get(challenges::get_sensor_status))
        .route("/api/health", get(health_check))
        .with_state(state);

    Ok(router.into())
}

// Returns a list of all unique tank IDs
async fn get_all_tanks(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let rows = sqlx::query("SELECT DISTINCT tank_id FROM tank_readings")
        .fetch_all(&state.pool)
        .await?;
    let tank_ids: Vec<String> = rows
        .into_iter()
        .map(|row| row.get::<String, _>("tank_id"))
        .collect();
    Ok(Json(tank_ids))
}

// This function has a blocking operation that's causing high latency
async fn get_tank_readings(
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

    // Read tank configuration file using blocking I/O
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
            data_found = false,
            "No sensor readings found for tank"
        );
        // For this example, we'll just return empty results
        // But in a real app, you might want to return a specific error:
        // return Err(ApiError::TankNotFound(format!("No readings for tank {}", tank_id)));
    }

    // Calculate the actual request duration
    let elapsed = start.elapsed().as_millis() as f64;

    // Custom metric showing request duration
    tracing::info!(
        request_id = %request_id,
        tank_id = %tank_id,
        request_duration_ms = elapsed,
        api_endpoint = "tank_readings",
        io_duration_ms = io_duration,
        db_duration_ms = db_duration,
        data_points_retrieved = readings.len(),
        operation_status = "success",
        "Tank environmental readings retrieved"
    );

    // Create a simplified response with readings, settings, and minimal metadata
    let response = serde_json::json!({
        "readings": readings,
        "settings_summary": settings,
        "metadata": {
            "tank_id": tank_id,
            "count": readings.len(),
            "latency_ms": elapsed  // Only include total request time for the frontend challenge
        }
    });

    Ok(Json(response))
}

// Challenge functions and implementations moved to challenges.rs

// Helper functions removed since we're using runtime validation instead of code analysis

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Aqua Monitor service is running")
}

/// Validates the implementation of Challenge #1: Async I/O
async fn validate_challenge_solution(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // Create a request ID for correlation in logs
    let request_id = uuid::Uuid::new_v4().to_string();
    tracing::info!(
        request_id = %request_id,
        "Starting validation for Challenge #1: Async I/O"
    );

    // Use environment variable to determine if async I/O is being used
    let is_valid = std::env::var("USING_ASYNC_IO").is_ok();

    // Create the response with appropriate feedback
    let response = json!({
        "valid": is_valid,
        "message": if is_valid {
            "Solution correctly implemented! Async I/O is now being used for file operations."
        } else {
            "Solution validation failed. Make sure you're using tokio::fs for file operations instead of std::fs, and remove any thread::sleep calls."
        },
        "system_component": {
            "name": "Tank Readings API",
            "description": if is_valid {
                "Tank readings API is now using async I/O operations"
            } else {
                "Tank readings API is experiencing high latency due to blocking I/O"
            },
            "status": if is_valid { "normal" } else { "degraded" }
        }
    });

    // Return the validation result
    (StatusCode::OK, Json(response))
}

/// Validates the implementation of Challenge #4: Resource Leak
async fn validate_resource_leak_solution(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("Starting validation for Challenge #4: Resource Leak");
    
    // Create a request ID for correlation in logs
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // For this challenge, we check if the implementation uses a static HTTP client
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    // Log the current directory for debugging
    tracing::info!(
        request_id = %request_id,
        current_dir = %current_dir.display(),
        "Current working directory for validation"
    );
    
    let source_path = current_dir.join("src/challenges.rs");
    
    // Log the full source path for debugging
    tracing::info!(
        request_id = %request_id,
        source_path = %source_path.display(),
        "Full source path for validation"
    );
    
    // Read the source code file
    let source_code = match fs::read_to_string(&source_path) {
        Ok(content) => content,
        Err(e) => {
            tracing::error!(
                request_id = %request_id,
                error = %e,
                "Failed to read source code for validation"
            );
            // If we can't read the source, assume the challenge is not completed
            return (StatusCode::OK, Json(json!({
                "valid": false,
                "message": "Validation failed: Unable to verify implementation.",
                "system_component": {
                    "name": "Sensor Status API",
                    "description": "Sensor status API is creating too many client instances",
                    "status": "degraded"
                }
            })));
        }
    };
    
    // Extract just the challenge code section using the challenge markers
    let challenge_start = source_code.find("// ⚠️ CHALLENGE #4: RESOURCE LEAK ⚠️");
    let challenge_end = source_code.find("// ⚠️ END CHALLENGE CODE ⚠️");
    
    // Check if we found the challenge section boundaries
    if challenge_start.is_none() || challenge_end.is_none() {
        tracing::error!(
            request_id = %request_id,
            "Could not find challenge section boundaries in source code"
        );
        return (StatusCode::OK, Json(json!({
            "valid": false,
            "message": "Validation failed: Unable to verify implementation.",
            "system_component": {
                "name": "Sensor Status API",
                "description": "Sensor status API is creating too many client instances",
                "status": "degraded"
            }
        })));
    }
    
    // Extract just the challenge code section
    let challenge_code = &source_code[challenge_start.unwrap()..challenge_end.unwrap() + "// ⚠️ END CHALLENGE CODE ⚠️".len()];
    
    // Check for module-level static HTTP client definition (outside challenge section)
    let has_static_client = source_code.contains("static HTTP_CLIENT") || 
                            source_code.contains("static CLIENT");
    
    // Check for use of static client instead of creating new client
    let uses_static_client = challenge_code.contains("&*HTTP_CLIENT") || 
                             challenge_code.contains("HTTP_CLIENT.") ||
                             challenge_code.contains("&*CLIENT") ||
                             challenge_code.contains("CLIENT.");
    
    // Check for absence of new client creation in challenge code
    let no_new_client = !challenge_code.contains("Client::new()") || 
                        (challenge_code.contains("Client::new()") && 
                         challenge_code.contains("Lazy::new"));
    
    // Log what we're finding in the challenge code
    tracing::info!(
        request_id = %request_id,
        has_static_client = has_static_client,
        uses_static_client = uses_static_client,
        no_new_client = no_new_client,
        "Challenge code check results"
    );
    
    // All checks must pass for validation to succeed
    let is_valid = has_static_client && uses_static_client && no_new_client;
    
    // Build a standardized response following the same format as other challenges
    let response = json!({
        "valid": is_valid,
        "message": if is_valid {
            "Solution correctly implemented! HTTP client is now shared and resource-efficient."
        } else {
            "Solution validation failed. Please implement a shared, static HTTP client instead of creating a new one for each request."
        },
        "system_component": {
            "name": "Sensor Status API",
            "description": if is_valid {
                "Sensor status API is now resource-efficient"
            } else {
                "Sensor status API is creating too many client instances"
            },
            "status": if is_valid { "normal" } else { "degraded" }
        }
    });
    
    (StatusCode::OK, Json(response))
}

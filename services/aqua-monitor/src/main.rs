mod challenges;

use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_axum::axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sqlx::{PgPool, Row};
use std::fs;
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
        .route("/api/tanks/:tank_id/readings", get(challenges::get_tank_readings))
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

    // Get the file content to check implementation patterns
    tracing::info!("Working directory: {:?}", std::env::current_dir());
    
    // Read the challenges.rs file where the implementation is
    let source_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("src/challenges.rs");
    
    // Log the full source path for debugging
    tracing::info!(
        request_id = %request_id,
        source_path = %source_path.display(),
        "Full source path for validation"
    );
    
    // Read the source code file
    let challenge_file = match fs::read_to_string(&source_path) {
        Ok(content) => content,
        Err(e) => {
            tracing::error!(
                request_id = %request_id,
                error = %e,
                "Failed to read source code for validation"
            );
            // If we can't read the source, assume the challenge is not completed
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "valid": false,
                "message": format!("Error reading source file: {}", e)
            })));
        }
    };
    
    // Extract the challenge code between the markers (case insensitive and allowing for spacing variations)
    // We look for both actual markers or comments mentioning them
    let markers = [
        "// ⚠️ CHALLENGE #1: ASYNC I/O ⚠️", 
        "// CHALLENGE #1: ASYNC I/O",
        "// Challenge 1: Async I/O",
        "// Challenge 1",
        "// CHALLENGE #1"
    ];
    
    let end_markers = [
        "// ⚠️ END CHALLENGE CODE ⚠️",
        "// END CHALLENGE CODE",
        "// End Challenge Code",
        "// End Challenge 1",
        "// END CHALLENGE #1"
    ];
    
    let challenge_start = markers.iter()
        .filter_map(|marker| challenge_file.find(marker))
        .min();
    
    let challenge_end = end_markers.iter()
        .filter_map(|marker| challenge_file.find(marker))
        .min();
    
    let challenge_code = match (challenge_start, challenge_end) {
        (Some(start), Some(end)) if start < end => {
            &challenge_file[start..end]
        },
        _ => {
            // If we can't find clear markers, just check the whole file
            // This is more flexible but still validates the important parts
            tracing::info!("Couldn't find clear challenge markers, checking whole file");
            &challenge_file
        }
    };
    
    // Print a snippet of the extracted challenge code for debugging
    let excerpt = if challenge_code.len() > 200 {
        &challenge_code[0..200]
    } else {
        challenge_code
    };
    tracing::info!("Challenge code excerpt: {}", excerpt);
    
    // Simple function to check if a pattern exists in uncommented code
    let is_uncommented = |pattern: &str| -> bool {
        challenge_code.lines()
            .filter(|line| !line.trim().starts_with("//"))
            .any(|line| line.contains(pattern))
    };
    
    // Check for key implementation patterns
    let uses_async_io = is_uncommented("tokio::fs") || 
                       is_uncommented("async_std::fs") || 
                       (is_uncommented(".await") && 
                        (is_uncommented("read_to_string") || is_uncommented("read_file")));
    
    // Check for absence of blocking operations
    let no_blocking_operations = !is_uncommented("std::thread::sleep") && 
                               !is_uncommented("std::fs::read");

    // Check for proper tracing implementation - be flexible about the exact approach
    let has_proper_tracing = (is_uncommented("tracing::info_span") || is_uncommented("info_span!")) &&
                            (is_uncommented(".in_scope") || is_uncommented(".instrument"));
    
    // Log the key findings
    tracing::info!(
        request_id = %request_id,
        uses_async_io = uses_async_io,
        no_blocking_operations = no_blocking_operations,
        has_proper_tracing = has_proper_tracing,
        "Challenge validation check results"
    );
    
    // All checks must pass for validation to succeed
    let is_valid = uses_async_io && no_blocking_operations && has_proper_tracing;

    // Create the response with appropriate feedback
    let response = json!({
        "valid": is_valid,
        "message": if is_valid {
            "Solution correctly implemented! Async I/O is now being used with proper tracing.".to_string()
        } else {
            let mut issues = Vec::new();
            if !uses_async_io {
                issues.push("Make sure you're using async file operations (e.g., tokio::fs)");
            }
            if !no_blocking_operations {
                issues.push("Remove any blocking operations (std::fs, thread::sleep)");
            }
            if !has_proper_tracing {
                issues.push("Ensure proper tracing implementation for async operations");
            }
            format!("Solution validation failed. Issues to address: {}", issues.join(", "))
        },
        "system_component": {
            "name": "Tank Readings API",
            "description": if is_valid {
                "Tank readings API is now using async I/O operations with proper tracing"
            } else {
                "Tank readings API is experiencing high latency due to blocking I/O or improper tracing"
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
    // or client in app state (also a valid approach)
    let has_static_client = source_code.contains("static HTTP_CLIENT") || 
                            source_code.contains("static CLIENT") ||
                            source_code.contains("client: reqwest::Client") ||
                            source_code.contains("http_client: reqwest::Client");
    
    // Simple function to check if a pattern exists in uncommented code
    let is_uncommented = |pattern: &str| -> bool {
        challenge_code.lines()
            .filter(|line| !line.trim().starts_with("//"))
            .any(|line| line.contains(pattern))
    };
    
    // Check for use of static client instead of creating new client
    // Allow more flexible usage patterns including direct CLIENT usage and &CLIENT (deref coercion)
    let uses_static_client = is_uncommented("&*HTTP_CLIENT") || 
                             is_uncommented("HTTP_CLIENT.") ||
                             is_uncommented("&*CLIENT") ||
                             is_uncommented("&CLIENT") ||
                             is_uncommented("CLIENT.") ||
                             is_uncommented("CLIENT") ||
                             is_uncommented("state.client") ||
                             is_uncommented("state.http_client");
    
    // No new client if either there's no Client::new() call at all, or it's only used with Lazy::new
    // or if it's only used once for initialization in the app state
    let no_new_client = !is_uncommented("Client::new()") || 
                        challenge_code.contains("Lazy::new") ||
                        (source_code.contains("client: reqwest::Client") && 
                         source_code.matches("Client::new()").count() <= 1);
    
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
            "Solution validation failed. Please implement a shared HTTP client using one of these approaches: 1) a static CLIENT with once_cell/lazy_static, 2) storing the client in AppState, or 3) another approach that avoids creating a new client for each request."
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

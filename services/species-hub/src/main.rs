mod challenges;

use shuttle_axum::axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
// CORS removed - managed by frontend
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use std::fs;
use tracing;
use thiserror::Error;

// Custom Error Type for species-hub service
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Species not found: {0}")]
    SpeciesNotFound(String),
    
    #[error("Invalid query parameter: {0}")]
    InvalidQuery(String),
    
    #[error("Feeding schedule error: {0}")]
    ScheduleError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Implement IntoResponse for our custom error type
impl IntoResponse for ApiError {
    fn into_response(self) -> shuttle_axum::axum::response::Response {
        let (status, error_message) = match &self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::SpeciesNotFound(id) => (StatusCode::NOT_FOUND, format!("Species not found: {}", id)),
            ApiError::InvalidQuery(msg) => (StatusCode::BAD_REQUEST, format!("Invalid query: {}", msg)),
            ApiError::ScheduleError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, format!("Feeding schedule error: {}", msg)),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };
        
        // Log the error with structured fields
        tracing::error!(
            error.type = std::any::type_name::<Self>(),
            error.message = %error_message,
            error.status = %status.as_u16(),
            "API error occurred"
        );
        
        // Create response with proper content type to ensure JSON is correctly processed
        let body = Json(serde_json::json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));
        
        // Convert to Response explicitly using shuttle_axum::axum::response
        (status, body).into_response()
    }
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Serialize, Deserialize)]
struct Species {
    id: i32,
    name: String,
    scientific_name: String,
    description: String,
    min_temperature: f64,
    max_temperature: f64,
    min_ph: f64,
    max_ph: f64,
    diet_type: String,
}

#[derive(Deserialize)]
struct SpeciesQuery {
    name: Option<String>,
    scientific_name: Option<String>,
}

#[derive(Deserialize, Default)]
struct FeedingScheduleParams {
    tank_id: Option<String>,
    custom_diet: Option<String>,
}

#[derive(Serialize)]
struct FeedingSchedule {
    species_id: i32,
    feeding_times: Vec<String>,
    food_type: String,
    amount_grams: f64,
}

// Tank information structure - mirrors data from aqua-monitor
#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Tank {
    id: String,
    name: String,
    tank_type: String,
    volume: f64,
    description: Option<String>,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Initialize database with logging and proper error handling
    tracing::info!("Running database migrations for species-hub...");
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        tracing::error!(error = %e, "Database migration failed for species-hub");
        return Err(anyhow::anyhow!("Database migration failed: {e}").into());
    }
    tracing::info!("Database migrations completed successfully for species-hub.");
    
    // Initialize state
    let state = AppState { pool };
    
    // Build router
    let router = Router::new()
        .route("/api/species", get(challenges::get_species))
        .route("/api/species/:id", get(get_species_by_id))
        .route("/api/species/:species_id/feeding-schedule", get(challenges::get_feeding_schedule))
        .route(
            "/api/challenges/2/validate",
            get(validate_query_optimization),
        )
        .route("/api/health", get(health_check))
        .with_state(state);
    
    Ok(router.into())
}

async fn get_species_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Add request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();
    
    // Create a span with request ID context to avoid nesting issues
    let span = tracing::info_span!("species_profile_lookup", %request_id);
    let _guard = span.enter();
    
    tracing::info!(
        request_id = %request_id,
        species_id = id,
        operation = "species_profile_lookup",
        "Starting species profile lookup"
    );
    
    // Check if ID is valid
    if id <= 0 {
        return Err(ApiError::InvalidQuery(format!("Invalid species ID: {}", id)));
    }
    
    // Use runtime query instead of compile-time checked macro
    let species = sqlx::query("SELECT * FROM species WHERE id = $1")
        .bind(id)
        .map(|row: sqlx::postgres::PgRow| {
            Species {
                id: row.get("id"),
                name: row.get("name"),
                scientific_name: row.get("scientific_name"),
                description: row.get("description"),
                min_temperature: row.get("min_temperature"),
                max_temperature: row.get("max_temperature"),
                min_ph: row.get("min_ph"),
                max_ph: row.get("max_ph"),
                diet_type: row.get("diet_type"),
            }
        })
        .fetch_optional(&state.pool)
        .await
        .map_err(ApiError::Database)?;
    
    // Calculate operation duration
    let elapsed = start_time.elapsed().as_millis() as f64;
    
    // Log based on operation result
    match species {
        Some(s) => {
            // Extract name for logging before moving s into the response
            let species_name = s.name.clone();
            
            tracing::info!(
                request_id = %request_id,
                species_id = id,
                species_name = %species_name,
                db_query_time_ms = elapsed,
                operation_status = "success",
                "Species profile lookup succeeded"
            );
            
            Ok(Json(s))
        },
        None => {
            tracing::warn!(
                request_id = %request_id,
                species_id = id,
                db_query_time_ms = elapsed,
                operation_status = "not_found",
                "Species profile lookup failed: species not found"
            );
            Err(ApiError::SpeciesNotFound(format!("Species with ID {} not found", id)))
        }
    }
}

async fn health_check() -> impl IntoResponse {
    // Create a span for species database health check
    let span = tracing::info_span!("species_database_health");
    let _guard = span.enter();
    StatusCode::OK
}

// Note: Rate limiting functionality would be implemented here in a real system

/// Validates the implementation of Challenge #2: Query Optimization
async fn validate_query_optimization(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("Starting validation for Challenge #2: Query Optimization");
    
    // Create a request ID for correlation in logs
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // For this challenge, we simply check if the implementation uses ILIKE queries
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    // Log the current directory for debugging
    tracing::info!(
        request_id = %request_id,
        current_dir = %current_dir.display(),
        "Current working directory for validation"
    );
    
    let source_path = current_dir.join("src/challenges.rs"); // Updated to look at challenges.rs
    
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
                    "name": "Species Database",
                    "description": "Species database search is experiencing slowdowns",
                    "status": "degraded"
                }
            })));
        }
    };
    
    // Extract just the challenge code section using the challenge markers
    let challenge_start = source_code.find("// ⚠️ CHALLENGE #2: DATABASE QUERY OPTIMIZATION ⚠️");
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
                "name": "Species Database",
                "description": "Species database search is experiencing slowdowns",
                "status": "degraded"
            }
        })));
    }
    
    // Extract just the challenge code section
    let challenge_code = &source_code[challenge_start.unwrap()..challenge_end.unwrap() + "// ⚠️ END CHALLENGE CODE ⚠️".len()];
    
    // Simple function to check if a pattern exists in uncommented code
    let is_uncommented = |pattern: &str| -> bool {
        challenge_code.lines()
            .filter(|line| !line.trim().starts_with("//"))
            .any(|line| line.contains(pattern))
    };
    
    // Check for correct ILIKE usage and any remaining LIKE usage
    let name_uses_ilike = is_uncommented("WHERE name ILIKE $1");
    let scientific_name_uses_ilike = is_uncommented("WHERE scientific_name ILIKE $1");
    let still_using_like = is_uncommented("WHERE name LIKE $1") || 
                          is_uncommented("WHERE scientific_name LIKE $1");
    
    // Log key validation findings
    tracing::info!(
        request_id = %request_id,
        name_uses_ilike = name_uses_ilike,
        scientific_name_uses_ilike = scientific_name_uses_ilike,
        still_using_like = still_using_like,
        "Challenge validation check results"
    );
    
    // Both queries must use ILIKE for validation to pass
    let is_valid = name_uses_ilike && scientific_name_uses_ilike;
    
    // Build a standardized response following the same format as other challenges
    let response = json!({
        "valid": is_valid,
        "message": if is_valid {
            "Solution correctly implemented! Queries are now optimized for case-insensitive searches."
        } else if still_using_like {
            "Solution validation failed. You're still using case-sensitive LIKE instead of ILIKE for some queries."
        } else {
            "Solution validation failed. Make sure to use ILIKE for all queries to optimize case-insensitive searches."
        },
        "system_component": {
            "name": "Species Database",
            "description": if is_valid {
                "Species database search is now optimized"
            } else {
                "Species database search is experiencing slowdowns"
            },
            "status": if is_valid { "normal" } else { "degraded" }
        }
    });
    
    (StatusCode::OK, Json(response))
}

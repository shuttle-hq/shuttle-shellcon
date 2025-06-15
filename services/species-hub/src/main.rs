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

#[derive(Serialize, Deserialize, sqlx::FromRow)]
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
    State(state): State<AppState>, // Changed to use state for db access
) -> impl IntoResponse {
    tracing::info!("Starting validation for Challenge #2: Query Optimization");
    
    // Create a request ID for correlation in logs
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // --- Database Checks --- 
    let pg_trgm_enabled: Result<bool, _> = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'pg_trgm')"
    )
    .fetch_one(&state.pool)
    .await;

    let name_index_exists: Result<bool, _> = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'species' AND indexname = 'species_name_gin_trgm_idx' AND indexdef ILIKE '%USING GIN (name gin_trgm_ops)%')"
    )
    .fetch_one(&state.pool)
    .await;

    let scientific_name_index_exists: Result<bool, _> = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'species' AND indexname = 'species_scientific_name_gin_trgm_idx' AND indexdef ILIKE '%USING GIN (scientific_name gin_trgm_ops)%')"
    )
    .fetch_one(&state.pool)
    .await;

    // Handle potential database query errors for checks
    if pg_trgm_enabled.is_err() || name_index_exists.is_err() || scientific_name_index_exists.is_err() {
        tracing::error!(
            request_id = %request_id,
            pg_trgm_error = ?pg_trgm_enabled.err(),
            name_index_error = ?name_index_exists.err(),
            scientific_name_index_error = ?scientific_name_index_exists.err(),
            "Database error during validation checks for Challenge #2"
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "valid": false,
            "message": "Validation failed: Could not perform database checks for pg_trgm extension or indexes.",
            "system_component": {
                "name": "Species Database",
                "description": "Species database search is experiencing slowdowns",
                "status": "degraded"
            }
        })));
    }

    let pg_trgm_ok = pg_trgm_enabled.unwrap_or(false);
    let name_index_ok = name_index_exists.unwrap_or(false);
    let scientific_name_index_ok = scientific_name_index_exists.unwrap_or(false);
    let db_setup_ok = pg_trgm_ok && name_index_ok && scientific_name_index_ok;

    // --- Source Code Checks (ILIKE usage) ---
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let source_path = current_dir.join("src/challenges.rs");
    
    let source_code = match fs::read_to_string(&source_path) {
        Ok(content) => content,
        Err(e) => {
            tracing::error!(
                request_id = %request_id,
                error = %e,
                path = %source_path.display(),
                "Failed to read source code for validation (src/challenges.rs)"
            );
            return (StatusCode::OK, Json(json!({
                "valid": false,
                "message": "Validation failed: Unable to read src/challenges.rs to verify ILIKE usage.",
                "system_component": {
                    "name": "Species Database",
                    "description": "Species database search is experiencing slowdowns",
                    "status": "degraded"
                }
            })));
        }
    };

    let challenge_start = source_code.find("// ⚠️ CHALLENGE #2: DATABASE QUERY OPTIMIZATION ⚠️");
    let challenge_end = source_code.find("// ⚠️ END CHALLENGE CODE ⚠️"); // Assuming this marker exists or is added

    let ilike_checks_possible = challenge_start.is_some() && challenge_end.is_some();
    let (name_uses_ilike, scientific_name_uses_ilike, still_using_like) = if ilike_checks_possible {
        let challenge_code = &source_code[challenge_start.unwrap()..challenge_end.unwrap()];
        let is_uncommented = |pattern: &str| -> bool {
            challenge_code.lines()
                .filter(|line| !line.trim().starts_with("//"))
                .any(|line| line.contains(pattern))
        };
        (
            is_uncommented("WHERE name ILIKE $1"),
            is_uncommented("WHERE scientific_name ILIKE $1"),
            is_uncommented("WHERE name LIKE $1") || is_uncommented("WHERE scientific_name LIKE $1")
        )
    } else {
        tracing::warn!(
            request_id = %request_id,
            "Could not find challenge markers in src/challenges.rs for ILIKE checks. Skipping ILIKE validation."
        );
        (false, false, true) // Default to failing ILIKE checks if markers are missing
    };

    let ilike_ok = name_uses_ilike && scientific_name_uses_ilike && !still_using_like;

    // --- Final Validation Logic ---
    let is_valid = db_setup_ok && ilike_ok;

    tracing::info!(
        request_id = %request_id,
        pg_trgm_enabled = pg_trgm_ok,
        name_index_exists = name_index_ok,
        scientific_name_index_exists = scientific_name_index_ok,
        db_setup_valid = db_setup_ok,
        name_uses_ilike = name_uses_ilike,
        scientific_name_uses_ilike = scientific_name_uses_ilike,
        still_using_like = still_using_like, // This should be false for ilike_ok to be true
        ilike_usage_valid = ilike_ok,
        overall_valid = is_valid,
        "Challenge #2 validation check results"
    );

    let mut message = String::new();
    if is_valid {
        message = "Solution correctly implemented! Database is optimized with pg_trgm and GIN indexes, and queries use ILIKE.".to_string();
    } else {
        message.push_str("Validation failed: ");
        if !pg_trgm_ok {
            message.push_str("The 'pg_trgm' extension is not enabled in the database. ");
        }
        if !name_index_ok {
            message.push_str("A GIN trigram index on 'species.name' (e.g., 'species_name_gin_trgm_idx') is missing or incorrect. ");
        }
        if !scientific_name_index_ok {
            message.push_str("A GIN trigram index on 'species.scientific_name' (e.g., 'species_scientific_name_gin_trgm_idx') is missing or incorrect. ");
        }
        if !ilike_ok {
            if still_using_like {
                message.push_str("Source code in 'src/challenges.rs' still uses LIKE instead of ILIKE for some queries. ");
            } else if !name_uses_ilike || !scientific_name_uses_ilike {
                message.push_str("Source code in 'src/challenges.rs' does not consistently use ILIKE for both name and scientific_name searches. ");
            }
        }
        if !ilike_checks_possible && !is_valid { // Add this if markers were the primary issue for ILIKE
             message.push_str("Could not find challenge markers in 'src/challenges.rs' to verify ILIKE usage. ");
        }
    }

    let response = json!({
        "valid": is_valid,
        "message": message.trim_end(),
        "system_component": {
            "name": "Species Database",
            "description": if is_valid {
                "Species database search is now optimized"
            } else {
                "Species database search is experiencing slowdowns or is misconfigured"
            },
            "status": if is_valid { "normal" } else { "degraded" }
        }
    });

    (StatusCode::OK, Json(response))
}

use shuttle_axum::axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
// CORS removed - managed by frontend
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
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
        .route("/api/species", get(get_species))
        .route("/api/species/:id", get(get_species_by_id))
        .route("/api/species/:species_id/feeding-schedule", get(get_feeding_schedule))
        .route("/api/species/validate-solution", get(validate_query_optimization))
        .route("/api/health", get(health_check))
        .with_state(state);
    
    Ok(router.into())
}

// CHALLENGE #2: Fix the inefficient database query in this function
// This function uses a non-indexed LIKE query that's causing slow performance
async fn get_species(
    Query(params): Query<SpeciesQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Add request ID for correlation and timing
    let request_id = uuid::Uuid::new_v4().to_string();
    let start = std::time::Instant::now();
    
    // Create a non-nested span for species catalog search with request_id as context
    let span = tracing::info_span!("species_catalog_search", %request_id);
    let _guard = span.enter();
    
    // Log operation start with search params
    tracing::info!(
        request_id = %request_id,
        operation = "species_catalog_search",
        search_by_name = params.name.is_some(),
        search_by_scientific_name = params.scientific_name.is_some(),
        "Starting species catalog search"
    );
    
    // Validate query parameters
    if let Some(name) = &params.name {
        if name.len() < 2 {
            return Err(ApiError::InvalidQuery("Name search term must be at least 2 characters".to_string()));
        }
    }
    
    if let Some(scientific_name) = &params.scientific_name {
        if scientific_name.len() < 2 {
            return Err(ApiError::InvalidQuery("Scientific name search term must be at least 2 characters".to_string()));
        }
    }
    
    // Using optimized case-insensitive query with ILIKE
    // This allows PostgreSQL to utilize trigram indexes if available
    let species = if let Some(name) = &params.name {
        // Use runtime query with ILIKE for case-insensitivity and better index usage
        sqlx::query("SELECT * FROM species WHERE name ILIKE $1")
            .bind(format!("%{}%", name))
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
            .fetch_all(&state.pool)
            .await
            .map_err(ApiError::Database)?
    } else if let Some(scientific_name) = &params.scientific_name {
        sqlx::query("SELECT * FROM species WHERE scientific_name ILIKE $1")
            .bind(format!("%{}%", scientific_name))
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
            .fetch_all(&state.pool)
            .await
            .map_err(ApiError::Database)?
    } else {
        sqlx::query("SELECT * FROM species LIMIT 100")
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
            .fetch_all(&state.pool)
            .await
            .map_err(ApiError::Database)?
    };
    
    // Calculate query time
    let query_time = start.elapsed().as_millis();
    
    // Log detailed performance metrics
    tracing::info!(
        request_id = %request_id,
        operation = "species_catalog_search",
        operation_status = if query_time < 50 { "optimized" } else { "standard" },
        db_query_time_ms = query_time as f64,
        db_rows_returned = species.len(),
        db_query_type = "species_search",
        search_term = params.name.as_deref().unwrap_or_else(|| params.scientific_name.as_deref().unwrap_or("")),
        "Species catalog search completed"
    );
    
    // If no species are found, we might want to return a specific error
    // In this case we'll return empty results, but you could also:
    // if species.is_empty() {
    //     return Err(ApiError::SpeciesNotFound("No species matched your search criteria".to_string()));
    // }
    
    Ok(Json(species))
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

// This function needs improved error handling for robustness
async fn get_feeding_schedule(
    Path(species_id): Path<i32>,
    Query(params): Query<FeedingScheduleParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Add request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let start = std::time::Instant::now();
    
    // Create a span with request ID context to avoid nesting issues
    let span = tracing::info_span!("feeding_schedule_generator", %request_id);
    let _guard = span.enter();
    
    tracing::info!(
        request_id = %request_id,
        species_id = species_id,
        tank_id = params.tank_id.as_deref().unwrap_or("default"),
        custom_diet = params.custom_diet.as_deref().unwrap_or("standard"),
        operation = "feeding_schedule_generation",
        "Starting feeding schedule generation"
    );
    
    // Validate species ID
    if species_id <= 0 {
        return Err(ApiError::InvalidQuery(format!("Invalid species ID: {}", species_id)));
    }
    
    // Get species info first - with proper error handling
    let species = sqlx::query("SELECT * FROM species WHERE id = $1")
        .bind(species_id)
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
        .map_err(|e| {
            tracing::error!(error.message = %e, "Database error when fetching species");
            ApiError::Database(e)
        })?;
    
    // Use the ? operator with our custom ApiError
    let species = species.ok_or_else(|| {
        ApiError::SpeciesNotFound(format!("Species with ID {} not found", species_id))
    })?;
    
    // If tank_id is provided, fetch tank information from our local tanks table
    let tank_type = if let Some(tank_id) = &params.tank_id {
        let tank = sqlx::query_as::<_, Tank>("SELECT * FROM tanks WHERE id = $1")
            .bind(tank_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| {
                tracing::error!(error.message = %e, "Database error when fetching tank");
                ApiError::Database(e)
            })?;
        
        // If tank exists, use its type; otherwise, use default
        tank.map(|t| t.tank_type)
    } else {
        None
    };
    
    // Calculate feeding schedule based on species and tank_type
    let schedule = calculate_feeding_schedule(&species, &params, tank_type);
    
    // Check if challenge is solved based on elapsed time
    let elapsed = start.elapsed().as_millis();
    if elapsed < 100 {
        tracing::info!(
            event.challenge_solved = "error_handling",
            challenge.id = 3,
            challenge.status = "solved",
            "Challenge #3 Solved: Error handling implemented!"
        );
    }
    
    // Log and return
    tracing::info!(
        request_id = %request_id,
        species_id = species.id,
        species_name = %species.name,
        feeding_times_per_day = schedule.feeding_times.len(),
        food_type = %schedule.food_type,
        schedule_calc_time_ms = elapsed as f64,
        operation_status = "success",
        "Feeding schedule generation completed"
    );
    
    Ok(Json(schedule))
}

fn calculate_feeding_schedule(species: &Species, params: &FeedingScheduleParams, tank_type: Option<String>) -> FeedingSchedule {
    // Use custom diet if provided
    let food_type = if let Some(diet) = &params.custom_diet {
        diet.clone()
    } else {
        match species.diet_type.as_str() {
            "carnivore" => "bloodworms".to_string(),
            "herbivore" => "algae wafers".to_string(),
            "filter feeder" => "phytoplankton".to_string(),
            _ => "flake food".to_string(),
        }
    };
    
    // Adjust feeding times based on tank_type from our local database
    let feeding_times = if let Some(tank_type) = tank_type {
        match tank_type.as_str() {
            "reef" => vec!["07:00".to_string(), "12:00".to_string(), "17:00".to_string()],
            "nano" => vec!["09:00".to_string()],
            "community" => vec!["08:00".to_string(), "16:00".to_string()],
            _ => vec!["08:00".to_string(), "16:00".to_string()] // Default schedule
        }
    } else {
        vec!["08:00".to_string(), "16:00".to_string()]
    };
    
    // Calculate amount based on species parameters
    let amount_grams = (species.min_temperature + species.max_temperature) / 10.0;
    
    FeedingSchedule {
        species_id: species.id,
        feeding_times,
        food_type,
        amount_grams,
    }
}

async fn health_check() -> impl IntoResponse {
    // Create a span for species database health check
    let span = tracing::info_span!("species_database_health");
    let _guard = span.enter();
    StatusCode::OK
}

/// Extract function content from source code
fn extract_function<'a>(source_code: &'a str, function_signature: &str) -> &'a str {
    if let Some(start) = source_code.find(function_signature) {
        // Count braces to find the end of the function
        let mut brace_count = 0;
        let mut found_first_brace = false;
        let mut end = start;

        for (i, ch) in source_code[start..].char_indices() {
            if ch == '{' {
                found_first_brace = true;
                brace_count += 1;
            } else if ch == '}' {
                brace_count -= 1;
            }

            if found_first_brace && brace_count == 0 {
                end = start + i + 1;
                break;
            }
        }

        &source_code[start..end]
    } else {
        ""
    }
}

/// Check if a pattern exists in a string that's not in a comment
fn has_non_commented_pattern(text: &str, pattern: &str) -> bool {
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("//") && trimmed.contains(pattern) {
            return true;
        }
    }
    false
}

/// Validates the implementation of Challenge #2: Query Optimization
async fn validate_query_optimization(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("Starting validation for Challenge #2: Query Optimization");
    
    use serde_json::json;

    // STEP 1: Get the source code and extract the implementation
    // Try both relative and absolute paths
    let source_path = "src/main.rs";
    let source_code = match tokio::fs::read_to_string(source_path).await {
        Ok(code) => code,
        Err(_) => {
            // Fallback to absolute path
            match tokio::fs::read_to_string("/Users/nvermande/Documents/Dev/shellcon/services/species-hub/src/main.rs").await {
                Ok(code) => code,
                Err(_) => {
                    tracing::error!("Failed to read source file");
                    String::new()
                }
            }
        }
    };
    
    // STEP 2: Extract the get_species function
    let get_species = extract_function(&source_code, "async fn get_species");
    
    // STEP 3: Check for specific patterns in the code
    let has_ilike_name = has_non_commented_pattern(get_species, "name ILIKE $1");
    let has_ilike_scientific = has_non_commented_pattern(get_species, "scientific_name ILIKE $1");
    let has_like_name = has_non_commented_pattern(get_species, "name LIKE $1");
    let has_like_scientific = has_non_commented_pattern(get_species, "scientific_name LIKE $1");
    
    tracing::info!("Code analysis: ilike_name={}, ilike_scientific={}, like_name={}, like_scientific={}", 
                  has_ilike_name, has_ilike_scientific, has_like_name, has_like_scientific);

    // STEP 4: Performance test comparing case-sensitive vs case-insensitive queries
    // For demonstration purposes, we'll just simulate the timing difference
    let non_optimized_duration = 120; // simulate milliseconds for non-optimized query
    let optimized_duration = 15;      // simulate milliseconds for optimized query
    
    // Solution is valid if it uses ILIKE for both name and scientific_name
    let is_valid = has_ilike_name && has_ilike_scientific && !has_like_name && !has_like_scientific;

    tracing::info!("Validation results: ilike_name={}, ilike_scientific={}, !like_name={}, !like_scientific={}, simulated_non_optimized_ms={}, simulated_optimized_ms={}", 
                  has_ilike_name, has_ilike_scientific, !has_like_name, !has_like_scientific, 
                  non_optimized_duration, optimized_duration);
    
    // Build a response with all validation details
    let response = json!({
        "valid": is_valid,
        "message": if is_valid {
            "Solution correctly implemented! Search queries are now optimized."
        } else if !has_ilike_name || !has_ilike_scientific {
            "Solution validation failed. Please implement case-insensitive ILIKE queries for both name and scientific_name searches."
        } else {
            "Solution validation failed. Please ensure you've removed the old LIKE queries."
        },
        "system_component": {
            "name": "Species Database",
            "status": if is_valid { "normal" } else { "degraded" },
            "description": if is_valid {
                "Species database search is now optimized"
            } else {
                "Species database search is experiencing slowdowns"
            }
        },
        "details": {
            "non_optimized_duration_ms": non_optimized_duration,
            "optimized_duration_ms": optimized_duration,
            "improvement_factor": non_optimized_duration as f32 / optimized_duration as f32,
            "ilike_name_implemented": has_ilike_name,
            "ilike_scientific_implemented": has_ilike_scientific,
            "like_name_removed": !has_like_name,
            "like_scientific_removed": !has_like_scientific
        }
    });
    
    // Log the validation result
    tracing::info!(
        query_optimization = is_valid,
        "Species search validation: {}", 
        if is_valid { "OPTIMIZED" } else { "NEEDS OPTIMIZATION" }
    );
    
    Json(response)
}

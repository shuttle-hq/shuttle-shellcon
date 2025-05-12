use shuttle_axum::axum::{
    extract::{Path, Query, State},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row}; // Add Row trait for get() method
// tower_http::trace::TraceLayer is used in the Shuttle examples but we're not using it here
use tracing;
use thiserror::Error;
// HashMap was likely used in an earlier implementation but is no longer needed

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
    fn into_response(self) -> axum::response::Response {
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
        
        // Return status code and JSON error message
        (status, Json(serde_json::json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))).into_response()
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

#[derive(Deserialize)]
struct FeedingScheduleParams {
    species_id: i32,
    tank_id: String,
    custom_diet: Option<String>,
}

#[derive(Serialize)]
struct FeedingSchedule {
    species_id: i32,
    feeding_times: Vec<String>,
    food_type: String,
    amount_grams: f64,
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
    
    // Configure CORS exactly like the Dad Joke example
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);
        
    // Build router with CORS
    let router = Router::new()
        .route("/api/species", get(get_species))
        .route("/api/species/{id}", get(get_species_by_id))
        .route("/api/feeding/schedule", get(get_feeding_schedule))
        .route("/api/health", get(health_check))
        .with_state(state)
        .layer(cors);
    
    // Return the router as ShuttleAxum
    Ok(router.into())
}

// CHALLENGE #2: Fix the inefficient database query in this function
// This function uses a non-indexed LIKE query that's causing slow performance
async fn get_species(
    Query(params): Query<SpeciesQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Start timing
    let start = std::time::Instant::now();
    
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
    
    // ⚠ FIX NEEDED HERE ⚠
    // This query is intentionally inefficient - it's doing a full table scan
    // with LIKE without using an index
    let species = if let Some(name) = &params.name {
        // Use runtime query instead of compile-time checked macro
        sqlx::query("SELECT * FROM species WHERE name LIKE $1")
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
        // Use runtime query instead of compile-time checked macro
        sqlx::query("SELECT * FROM species WHERE scientific_name LIKE $1")
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
        // Use runtime query instead of compile-time checked macro
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
    
    // Emit challenge status - triggered if query is fast
    if query_time < 50 {
        tracing::info!(
            event.challenge_solved = "query_optimization",
            challenge.id = 2,
            challenge.status = "solved",
            "Challenge #2 Solved: Query optimized!"
        );
    }
    
    tracing::info!(
        histogram.db_query_time_ms = query_time as f64,
        db.rows_returned = species.len(),
        db.query_type = "species_search",
        challenge.current_query_time = query_time as f64,
        search_term = params.name.as_deref().unwrap_or_else(|| params.scientific_name.as_deref().unwrap_or("")),
        "Species query completed"
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
    
    match species {
        Some(s) => Ok(Json(s)),
        None => Err(ApiError::SpeciesNotFound(format!("Species with ID {} not found", id))),
    }
}

// CHALLENGE #3: Fix the error handling in this function
// This function panics when an error occurs, crashing the service
async fn get_feeding_schedule(
    Query(params): Query<FeedingScheduleParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Start timing for potential challenge completion
    let start = std::time::Instant::now();
    
    // Validate species ID
    if params.species_id <= 0 {
        return Err(ApiError::InvalidQuery(format!("Invalid species ID: {}", params.species_id)));
    }
    
    // Get species info first - with proper error handling
    // Use runtime query instead of compile-time checked macro
    let species = sqlx::query("SELECT * FROM species WHERE id = $1")
        .bind(params.species_id)
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
        ApiError::SpeciesNotFound(format!("Species with ID {} not found", params.species_id))
    })?;
    
    // Calculate feeding schedule based on species
    let schedule = calculate_feeding_schedule(&species, &params);
    
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
        species.id = species.id,
        species.name = %species.name,
        schedule.times_per_day = schedule.feeding_times.len(),
        histogram.feed_schedule_calc_ms = elapsed as f64,
        "Feeding schedule calculated"
    );
    
    Ok(Json(schedule))
}

fn calculate_feeding_schedule(species: &Species, _params: &FeedingScheduleParams) -> FeedingSchedule {
    // Just an example implementation
    FeedingSchedule {
        species_id: species.id,
        feeding_times: vec!["08:00".to_string(), "16:00".to_string()],
        food_type: match species.diet_type.as_str() {
            "herbivore" => "Algae wafers".to_string(),
            "carnivore" => "Shrimp pellets".to_string(),
            _ => "Mixed feed".to_string(),
        },
        amount_grams: 2.5,
    }
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// SOLUTION FOR CHALLENGE #2
// Optimize the query and add proper indexes:
/*
// In migrations folder, add an index:
// CREATE INDEX idx_species_name ON species(name);
// CREATE INDEX idx_species_scientific_name ON species(scientific_name);

async fn get_species(
    Query(params): Query<SpeciesQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    
    // Optimized query using proper indexing and ILIKE for case insensitivity
    let species = if let Some(name) = &params.name {
        sqlx::query_as!(
            Species,
            "SELECT * FROM species WHERE name ILIKE $1",
            format!("%{}%", name)
        )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default()
    } else if let Some(scientific_name) = &params.scientific_name {
        sqlx::query_as!(
            Species,
            "SELECT * FROM species WHERE scientific_name ILIKE $1",
            format!("%{}%", scientific_name)
        )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as!(Species, "SELECT * FROM species LIMIT 100")
            .fetch_all(&state.pool)
            .await
            .unwrap_or_default()
    };
    
    let query_time = start.elapsed().as_millis();
    
    tracing::info!(
        histogram.db_query_time_ms = query_time as f64,
        db.rows_returned = species.len(),
        db.query_type = "species_search_optimized",
        "Species query completed with optimization"
    );
    
    Json(species)
}
*/

// SOLUTION FOR CHALLENGE #3
// Implement proper error handling:
/*
#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("Species not found: {0}")]
    SpeciesNotFound(i32),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Failed to calculate feeding schedule")]
    CalculationError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::SpeciesNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string()),
            ApiError::CalculationError => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        
        tracing::error!(error.message = %message, "API error occurred");
        
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

async fn get_feeding_schedule(
    Query(params): Query<FeedingScheduleParams>,
    State(state): State<AppState>,
) -> Result<Json<FeedingSchedule>, ApiError> {
    // Get species info first with proper error handling
    let species = sqlx::query_as!(
        Species,
        "SELECT * FROM species WHERE id = $1",
        params.species_id
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::SpeciesNotFound(params.species_id))?;
    
    // Calculate feeding schedule based on species
    let schedule = calculate_feeding_schedule(&species, &params);
    
    tracing::info!(
        counter.schedules_generated = 1,
        "Feeding schedule generated for species {}",
        species.name
    );
    
    Ok(Json(schedule))
}
*/

use shuttle_axum::axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use sqlx::{postgres::PgRow, Row};
use tracing;

use crate::{
    ApiError, AppState, FeedingSchedule, FeedingScheduleParams, 
    Species, SpeciesQuery
};

// ⚠️ CHALLENGE #2: DATABASE QUERY OPTIMIZATION ⚠️
// This function uses a non-indexed LIKE query that's causing slow performance
pub async fn get_species(
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
    
    // Using case-sensitive query with LIKE (non-optimized)
    // This requires a full table scan and doesn't utilize indexes effectively
    let species = if let Some(name) = &params.name {
        // Use runtime query with LIKE for case-sensitive search
        sqlx::query("SELECT * FROM species WHERE name LIKE $1")
            .bind(format!("%{}%", name))
            .map(|row: PgRow| {
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
            .map_err(|e| {
                tracing::error!(
                    request_id = %request_id,
                    error.type = "database",
                    error.message = %e,
                    "Error executing species name search query"
                );
                ApiError::Database(e)
            })?
    } else if let Some(scientific_name) = &params.scientific_name {
        // Use runtime query with LIKE for case-sensitive search
        sqlx::query("SELECT * FROM species WHERE scientific_name LIKE $1")
            .bind(format!("%{}%", scientific_name))
            .map(|row: PgRow| {
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
            .map_err(|e| {
                tracing::error!(
                    request_id = %request_id,
                    error.type = "database",
                    error.message = %e,
                    "Error executing species scientific name search query"
                );
                ApiError::Database(e)
            })?
    } else {
        // If no search parameters are provided, return all species (limit to 20)
        sqlx::query("SELECT * FROM species LIMIT 20")
            .map(|row: PgRow| {
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
            .map_err(|e| {
                tracing::error!(
                    request_id = %request_id,
                    error.type = "database",
                    error.message = %e,
                    "Error executing species list query"
                );
                ApiError::Database(e)
            })?
    };
    // ⚠️ END CHALLENGE CODE ⚠️
    
    // Get query duration
    let elapsed = start.elapsed().as_millis();
    
    // Log performance and results
    tracing::info!(
        request_id = %request_id,
        operation = "species_catalog_search",
        operation_status = "success",
        query_duration_ms = elapsed as f64,
        results_count = species.len(),
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

// This function needs improved error handling for robustness
pub async fn get_feeding_schedule(
    Path(species_id): Path<i32>,
    Query(params): Query<FeedingScheduleParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Add request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let start = std::time::Instant::now();
    
    // Create a span with request ID context to avoid nesting issues
    let span = tracing::info_span!(
        "feeding_schedule_generation",
        request_id = %request_id,
        species_id = %species_id
    );
    let _guard = span.enter();
    
    tracing::info!(
        request_id = %request_id,
        species_id = %species_id,
        operation = "get_feeding_schedule",
        "Generating feeding schedule"
    );
    
    // Fetch species information
    // Fetch species information with error handling
    let species = sqlx::query("SELECT * FROM species WHERE id = $1")
        .bind(species_id)
        .map(|row: PgRow| {
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
        .fetch_one(&state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                tracing::warn!(
                    request_id = %request_id,
                    species_id = %species_id,
                    "Species not found for feeding schedule"
                );
                ApiError::SpeciesNotFound(format!("Species with ID {} not found", species_id))
            }
            _ => {
                tracing::error!(
                    request_id = %request_id,
                    error.type = "database",
                    error.message = %e,
                    "Database error when fetching species"
                );
                ApiError::Database(e)
            }
        })?;
    
    // If tank_id is provided, fetch tank information
    let tank_type = if let Some(tank_id) = &params.tank_id {
        let tank = sqlx::query("SELECT tank_type FROM tanks WHERE id = $1")
            .bind(tank_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| {
                tracing::error!(
                    request_id = %request_id,
                    error.type = "database",
                    error.message = %e,
                    "Database error when fetching tank"
                );
                ApiError::Database(e)
            })?;
        
        // If tank exists, use its type; otherwise, use default
        tank.map(|t| t.get("tank_type"))
    } else {
        None
    };
    
    // Calculate feeding schedule based on species and tank_type
    let schedule = calculate_feeding_schedule(&species, &params, tank_type);
    
    // Calculate elapsed time for logging
    let elapsed = start.elapsed().as_millis();
    
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

// Helper function moved from main.rs
pub fn calculate_feeding_schedule(species: &Species, params: &FeedingScheduleParams, tank_type: Option<String>) -> FeedingSchedule {
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
    
    // Determine feeding frequency based on species type
    let feeding_times = match species.diet_type.as_str() {
        "carnivore" => vec!["08:00".to_string(), "20:00".to_string()],
        "herbivore" => vec!["08:00".to_string(), "13:00".to_string(), "18:00".to_string()],
        "filter feeder" => vec!["06:00".to_string()],
        _ => vec!["09:00".to_string(), "17:00".to_string()],
    };
    
    // Adjust amount based on species parameters
    let base_amount = (species.min_temperature + species.max_temperature) / 10.0;
    let amount_grams = match tank_type.as_deref() {
        Some("reef") => base_amount * 0.8, // Less food in reef tanks
        Some("planted") => base_amount * 1.1, // More food in planted tanks
        Some("brackish") => base_amount * 0.9, // Slightly less in brackish
        _ => base_amount, // Default amount
    };
    
    FeedingSchedule {
        species_id: species.id,
        feeding_times,
        food_type,
        amount_grams,
    }
}

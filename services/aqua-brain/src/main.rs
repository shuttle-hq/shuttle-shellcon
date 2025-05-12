use shuttle_axum::axum::{
    extract::{Query, State},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
// TokioMutex was likely used in an earlier implementation or is kept for a future solution
use tracing;
use thiserror::Error;
// TraceLayer is typically used in Shuttle examples but we're not using it here

// Define a custom error type for better error handling
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("System status unavailable: {0}")]
    SystemStatusUnavailable(String),
    
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),
    
    #[error("Species data unavailable: {0}")]
    SpeciesDataUnavailable(String),
    
    #[error("External service error: {0}")]
    ExternalService(#[from] reqwest::Error),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Implement IntoResponse for our custom error type
impl IntoResponse for ApiError {
    fn into_response(self) -> shuttle_axum::axum::response::Response {
        let (status, error_message) = match &self {
            ApiError::SystemStatusUnavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ApiError::AnalysisFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::SpeciesDataUnavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ApiError::ExternalService(_) => (StatusCode::BAD_GATEWAY, "External service error".to_string()),
            ApiError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
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
    monitor_client: reqwest::Client,
}

#[derive(Serialize, Deserialize, Clone)]
struct SystemStatus {
    environmental_monitoring: String, // "online", "degraded", "offline"
    species_database: String,
    feeding_system: String,
    remote_monitoring: String,
    analysis_engine: String,
    overall_status: String,
    last_updated: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AnalysisResult {
    tank_id: String,
    species_id: i32,
    timestamp: String,
    temperature_status: String,
    ph_status: String,
    oxygen_status: String,
    feeding_status: String,
    overall_health: String,
    recommendations: Vec<String>,
}

#[derive(Deserialize)]
struct AnalysisParams {
    tank_id: Option<String>,
    species_id: Option<i32>,
    timeframe: Option<String>,
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // Initialize clients
    let monitor_client = reqwest::Client::new();
    
    // Initialize state
    let state = AppState {
        monitor_client,
    };
    
    // Configure CORS exactly like the Dad Joke example
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);
        
    // Build router with CORS
    let router = Router::new()
        .route("/api/system/status", get(get_system_status))
        .route("/api/analysis", get(analyze_tank_conditions))
        .route("/api/challenges/current", get(get_current_challenge))
        .route("/api/health", get(health_check))
        .with_state(state)
        .layer(cors);
    
    // Return the router as ShuttleAxum
    Ok(router.into())
}

async fn get_system_status(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // In a real application, we would actually query the other services
    // For demonstration, we'll simulate potential errors
    
    // Start timing the request
    let start = std::time::Instant::now();
    
    // Simulate a call to the monitor service
    let _monitor_status = match state.monitor_client
        .get("http://localhost:8001/api/health") // This would be the real endpoint
        .timeout(std::time::Duration::from_secs(1))
        .send()
        .await {
            Ok(response) if response.status().is_success() => "online",
            Ok(_) => return Err(ApiError::SystemStatusUnavailable("Monitor service returned an error".into())),
            Err(e) => {
                // Show how errors are propagated with From<reqwest::Error>
                // This will be automatically converted to ApiError::ExternalService
                return Err(e.into());
            }
        };
    
    // For the example, we'll return mock data but with proper error handling
    let status = SystemStatus {
        environmental_monitoring: "degraded".to_string(),
        species_database: "degraded".to_string(),
        feeding_system: "offline".to_string(),
        remote_monitoring: "degraded".to_string(),
        analysis_engine: "degraded".to_string(),
        overall_status: "critical".to_string(),
        last_updated: chrono::Utc::now().to_rfc3339(),
    };
    
    // Calculate request time
    let elapsed = start.elapsed().as_millis() as f64;
    
    // Log structured metrics
    tracing::info!(
        counter.status_checks = 1,
        system.overall_status = status.overall_status,
        histogram.request_duration_ms = elapsed,
        "System status retrieved"
    );
    
    // Return success with the data
    Ok(Json(status))
}

// CHALLENGE #5: Fix the concurrency issue in this function
// This function has a global mutex that's causing high contention
async fn analyze_tank_conditions(
    Query(params): Query<AnalysisParams>,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // ⚠ FIX NEEDED HERE ⚠
    // This is intentionally inefficient - it's using a global mutex
    // for the entire analysis process, causing a concurrency bottleneck
    
    // Use once_cell for a properly initialized static HashMap
    use once_cell::sync::Lazy;
    static ANALYSIS_CACHE: Lazy<Mutex<HashMap<String, AnalysisResult>>> = 
        Lazy::new(|| Mutex::new(HashMap::new()));
    
    // Start timing
    let start = std::time::Instant::now();
    
    // Create cache key from parameters
    let cache_key = format!(
        "tank:{}_species:{}_time:{}",
        params.tank_id.as_deref().unwrap_or("all"),
        params.species_id.unwrap_or(0),
        params.timeframe.as_deref().unwrap_or("24h")
    );
    
    // Inefficient: locks the entire cache for the duration of the analysis
    let mut cache = ANALYSIS_CACHE.lock().unwrap();
    
    // Check if result is cached
    if let Some(result) = cache.get(&cache_key) {
        // Clone the result while we have the lock
        let result_clone = result.clone();
        // Release mutex early - but this doesn't help much because
        // the lock acquisition is still a bottleneck
        drop(cache);
        return Json(result_clone);
    }
    
    // Simulate fetching data from other services
    // In a real app, we would make actual API calls
    
    // Generate analysis result
    let result = AnalysisResult {
        tank_id: params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string()),
        species_id: params.species_id.unwrap_or(1),
        timestamp: chrono::Utc::now().to_rfc3339(),
        temperature_status: "warning".to_string(),
        ph_status: "critical".to_string(),
        oxygen_status: "normal".to_string(),
        feeding_status: "overdue".to_string(),
        overall_health: "at_risk".to_string(),
        recommendations: vec![
            "Reduce temperature by 2°C".to_string(),
            "Adjust pH to 7.2-7.5 range".to_string(),
            "Administer emergency feeding".to_string(),
        ],
    };
    
    // Cache the result - still holding the lock!
    cache.insert(cache_key.clone(), result.clone());
    
    // Calculate total time
    let analysis_time = start.elapsed().as_millis();
    
    // Emit challenge status based on analysis time
    // (Will trigger when concurrency issue is solved)
    if analysis_time < 50 {
        tracing::info!(
            event.challenge_solved = "concurrency",
            challenge.id = 5,
            challenge.status = "solved",
            mutex.contention = "eliminated",
            "Challenge #5 Solved: Concurrency bottleneck eliminated!"
        );
    }
    
    // Custom metric to track analysis performance with real measurement
    tracing::info!(
        histogram.analysis_time_ms = analysis_time as f64,
        tank.id = result.tank_id,
        analysis.health = result.overall_health,
        challenge.current_analysis_time = analysis_time as f64,
        "Tank analysis completed"
    );
    
    Json(result)
}

async fn get_current_challenge() -> impl IntoResponse {
    // This endpoint now returns all challenges and their statuses
    // It can be used by the frontend to display challenge progress
    
    // In a production app, this would query a database
    // For this example, we're returning static challenge descriptions
    // with dynamic status based on metrics
    
    // Get the current metrics (in a real app, these would come from your metrics system)
    // Here we're using dummy values that you can manually update when testing
    let latency_solved = std::env::var("CHALLENGE_1_SOLVED").is_ok();
    let query_solved = std::env::var("CHALLENGE_2_SOLVED").is_ok();
    let error_solved = std::env::var("CHALLENGE_3_SOLVED").is_ok();
    let resource_solved = std::env::var("CHALLENGE_4_SOLVED").is_ok();
    let concurrency_solved = std::env::var("CHALLENGE_5_SOLVED").is_ok();
    
    // Log event for dashboard tracking
    tracing::info!(
        event.challenges_status_check = 1,
        challenge.latency_solved = latency_solved,
        challenge.query_solved = query_solved,
        challenge.error_solved = error_solved,
        challenge.resource_solved = resource_solved,
        challenge.concurrency_solved = concurrency_solved,
        "Challenge status check"
    );
    
    Json(serde_json::json!({
        "challenges": [
            {
                "id": 1,
                "name": "latency-mystery",
                "title": "The Sluggish Sensor",
                "description": "The environmental monitoring system is experiencing severe delays, preventing timely readings of tank conditions. Investigate the high latency in the tank readings API!",
                "hint": "Check the sensor data retrieval function in the aqua-monitor service. Is there a blocking operation in an async context?",
                "service": "aqua-monitor",
                "file": "src/main.rs",
                "function": "get_tank_readings",
                "status": if latency_solved { "solved" } else { "pending" }
            },
            {
                "id": 2,
                "name": "query-conundrum",
                "title": "The Query Conundrum",
                "description": "The species database is responding slowly to searches. Optimize the query to improve performance.",
                "hint": "Check the species search function. Is the LIKE query optimized? Could you use a better index?",
                "service": "species-hub",
                "file": "src/main.rs",
                "function": "get_species",
                "status": if query_solved { "solved" } else { "pending" }
            },
            {
                "id": 4,
                "name": "resource-leak",
                "title": "The Leaky Connection",
                "description": "The sensor status API is creating a new HTTP client for every request, causing a resource leak.",
                "hint": "Create a static client instead of a new one for each request.",
                "service": "aqua-monitor",
                "file": "src/main.rs",
                "function": "get_sensor_status",
                "status": if resource_solved { "solved" } else { "pending" }
            },
            {
                "id": 5,
                "name": "concurrency-bottleneck",
                "title": "The Mutex Gridlock",
                "description": "The analysis engine has a severe concurrency bottleneck due to a global mutex.",
                "hint": "Replace the global mutex with a more granular locking strategy or a lock-free approach.",
                "service": "aqua-brain",
                "file": "src/main.rs",
                "function": "analyze_tank_conditions",
                "status": if concurrency_solved { "solved" } else { "pending" }
            }
        ],
        "total": 4,
        "solved": (if latency_solved { 1 } else { 0 }) + 
                 (if query_solved { 1 } else { 0 }) + 
                 (if resource_solved { 1 } else { 0 }) + 
                 (if concurrency_solved { 1 } else { 0 })
    }))
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// SOLUTION FOR CHALLENGE #5
// Fix the concurrency issue with a better caching strategy:
/*
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

// Better approach: use a RwLock for the cache to allow multiple readers
static ANALYSIS_CACHE: Lazy<RwLock<HashMap<String, (AnalysisResult, chrono::DateTime<chrono::Utc>)>>> = 
    Lazy::new(|| RwLock::new(HashMap::new()));

async fn analyze_tank_conditions(
    Query(params): Query<AnalysisParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    
    // Create cache key
    let cache_key = format!(
        "tank:{}_species:{}_time:{}",
        params.tank_id.as_deref().unwrap_or("all"),
        params.species_id.unwrap_or(0),
        params.timeframe.as_deref().unwrap_or("24h")
    );
    
    // Acquire read lock to check cache - allows concurrent readers
    let cache = ANALYSIS_CACHE.read().await;
    
    // Check if result is cached and not expired
    if let Some((result, timestamp)) = cache.get(&cache_key) {
        let age = chrono::Utc::now() - *timestamp;
        
        // Use cached result if less than 5 minutes old
        if age < chrono::Duration::minutes(5) {
            drop(cache); // Release read lock
            
            tracing::info!(
                counter.cache_hits = 1,
                cache.key = &cache_key,
                "Analysis cache hit"
            );
            
            return Json(result.clone());
        }
    }
    
    // Release read lock before acquiring write lock to prevent deadlock
    drop(cache);
    
    // Generate new analysis result
    // In a real app, this would call other services
    
    let result = AnalysisResult {
        tank_id: params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string()),
        species_id: params.species_id.unwrap_or(1),
        timestamp: chrono::Utc::now().to_rfc3339(),
        temperature_status: "warning".to_string(),
        ph_status: "critical".to_string(),
        oxygen_status: "normal".to_string(),
        feeding_status: "overdue".to_string(),
        overall_health: "at_risk".to_string(),
        recommendations: vec![
            "Reduce temperature by 2°C".to_string(),
            "Adjust pH to 7.2-7.5 range".to_string(),
            "Administer emergency feeding".to_string(),
        ],
    };
    
    // Acquire write lock to update cache
    let mut cache = ANALYSIS_CACHE.write().await;
    cache.insert(cache_key.clone(), (result.clone(), chrono::Utc::now()));
    drop(cache); // Release write lock as soon as possible
    
    let analysis_time = start.elapsed().as_millis();
    
    tracing::info!(
        histogram.analysis_time_ms = analysis_time as f64,
        tank.id = result.tank_id,
        analysis.health = result.overall_health,
        "Tank analysis completed (cache miss)"
    );
    
    Json(result)
}
*/

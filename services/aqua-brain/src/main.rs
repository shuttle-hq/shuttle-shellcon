use shuttle_axum::axum::extract::{Path, Query, State};
use shuttle_axum::axum::http::{HeaderValue, Method, StatusCode};
use shuttle_axum::axum::response::IntoResponse;
use shuttle_axum::axum::routing::get;
use shuttle_axum::axum::Json;
use shuttle_axum::axum::Router;
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
// No unused imports
use tracing;
use thiserror::Error;

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

#[derive(Deserialize, Clone)]
struct AnalysisParams {
    tank_id: Option<String>,
    species_id: Option<i32>,
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
        .route("/api/analysis/tanks", get(get_all_tank_analysis))
        .route("/api/analysis/tanks/:tank_id", get(get_tank_analysis_by_id))
        .route("/api/challenges/current", get(get_current_challenge))
        .route("/api/health", get(health_check))
        .with_state(state)
        .layer(cors);
    
    Ok(router.into())
}

async fn get_system_status(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let start = std::time::Instant::now();
        let _monitor_status = match state.monitor_client
        .get("http://localhost:8001/api/health")
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

async fn get_current_challenge() -> impl IntoResponse {
    // This endpoint now returns all challenges and their statuses
    // It can be used by the frontend to display challenge progress
    
    // Check if challenges are solved by doing timing measurements
    // These would normally check the actual database or logs
    // but for demo purposes, we'll just use hardcoded values
    let latency_solved = false; // Will be true when tank_readings < 100ms
    let query_solved = false;   // Will be true when species search is optimized
    let resource_solved = false; // Will be true when the sensor status uses a static client
    // Challenge #5 has been updated to RESTful API structure and is already solved
    // Log event for dashboard tracking
    tracing::info!(
        event.challenges_status_check = 1,
        challenge.latency_solved = latency_solved,
        challenge.query_solved = query_solved,
        challenge.resource_solved = resource_solved,
        challenge.api_structure_solved = true,
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
                "name": "api-structure",
                "title": "RESTful Endpoints",
                "description": "The analysis API has been restructured to follow RESTful conventions.",
                "hint": "Use /api/analysis/tanks for all tanks and /api/analysis/tanks/:id for a specific tank.",
                "service": "aqua-brain",
                "file": "src/main.rs",
                "function": "get_tank_analysis_by_id, get_all_tank_analysis",
                "status": "solved"
            }
        ],
        "total": 4,
        "solved": (if latency_solved { 1 } else { 0 }) + 
                 (if query_solved { 1 } else { 0 }) + 
                 (if resource_solved { 1 } else { 0 }) + 
                 1 // Challenge #5 is now always solved
    }))
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// Define a summary struct for collection response
#[derive(Serialize)]
struct TankSummary {
    tank_id: String,
    species_id: i32,
    species_name: String,
    overall_health: String,
    timestamp: String,
}

// Map species_id to species_name for the demo
fn get_species_name(species_id: i32) -> String {
    match species_id {
        1 => "Neon Tetra".to_string(),
        2 => "Clownfish".to_string(),
        3 => "Blue Tang".to_string(),
        4 => "Guppy".to_string(),
        5 => "Betta".to_string(),
        _ => format!("Unknown Species (ID: {})", species_id),
    }
}

// Handler for all tanks analysis - returns summarized information
async fn get_all_tank_analysis(
    Query(params): Query<AnalysisParams>,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // Defined tank IDs in our system
    let tank_ids = vec!["Tank-A1", "Tank-B2", "Tank-C3"];
    
    // Create summary results for all defined tanks
    let results: Vec<TankSummary> = tank_ids
        .into_iter()
        .map(|tank_id| {
            let mut tank_params = params.clone();
            tank_params.tank_id = Some(tank_id.to_string());
            
            // Get full analysis but only return summary
            let full_analysis = get_analysis_result(tank_params);
            
            // Convert to summary
            TankSummary {
                tank_id: full_analysis.tank_id,
                species_id: full_analysis.species_id,
                species_name: get_species_name(full_analysis.species_id),
                overall_health: full_analysis.overall_health,
                timestamp: full_analysis.timestamp,
            }
        })
        .collect();
        
    Json(results)
}

// Handler for single tank analysis by ID
async fn get_tank_analysis_by_id(
    State(_state): State<AppState>,
    Path(tank_id): Path<String>,
    Query(params): Query<AnalysisParams>,
) -> impl IntoResponse {
    // Override tank_id from path parameter
    let mut tank_params = params;
    tank_params.tank_id = Some(tank_id);
    
    // Get single tank analysis
    Json(get_analysis_result(tank_params))
}

// Helper function to generate analysis result (extracted from analyze_tank_conditions)
fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // Get tank_id or default to Tank-A1
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string());
    
    // Generate analysis result based on tank ID
    match tank_id.as_str() {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id,
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
        },
        "Tank-B2" => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(2),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "normal".to_string(),
            ph_status: "normal".to_string(),
            oxygen_status: "low".to_string(),
            feeding_status: "normal".to_string(),
            overall_health: "good".to_string(),
            recommendations: vec![
                "Increase aeration slightly".to_string(),
                "Monitor oxygen levels daily".to_string(),
            ],
        },
        "Tank-C3" => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(3),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "normal".to_string(),
            ph_status: "high".to_string(),
            oxygen_status: "normal".to_string(),
            feeding_status: "excess".to_string(),
            overall_health: "caution".to_string(),
            recommendations: vec![
                "Reduce feeding frequency".to_string(),
                "Perform 25% water change".to_string(),
                "Test ammonia levels".to_string(),
            ],
        },
        _ => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "unknown".to_string(),
            ph_status: "unknown".to_string(),
            oxygen_status: "unknown".to_string(),
            feeding_status: "unknown".to_string(),
            overall_health: "unknown".to_string(),
            recommendations: vec![
                "Verify tank ID".to_string(),
                "Setup monitoring system".to_string(),
            ],
        },
    }
}

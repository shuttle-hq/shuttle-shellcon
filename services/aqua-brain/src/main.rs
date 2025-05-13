use shuttle_axum::axum::extract::{Path, Query, State};
use shuttle_axum::axum::http::{HeaderValue, Method, StatusCode};
use shuttle_axum::axum::response::IntoResponse;
use shuttle_axum::axum::routing::{get, post};
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

#[derive(Deserialize)]
struct Challenge1SolutionRequest {
    code: String,
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
        .route("/api/challenges/test/1", get(test_challenge_1))
        .route("/api/challenges/1/solution", post(test_challenge_1_solution))
        .route("/api/health", get(health_check))
        .with_state(state)
        .layer(cors);
    
    Ok(router.into())
}

// Function to check if Challenge #1 (latency issue) is solved
async fn check_challenge_1_status() -> bool {
    // No span here since this is called from other functions
    tracing::info!("Checking sensor response time optimization status");
    
    // Make an API call to check if the solution is valid
    // This directly validates the current implementation rather than depending on file state
    let client = reqwest::Client::new();
    
    match client.get("http://localhost:8000/api/tanks/Tank-A1/validate-solution")
              .timeout(std::time::Duration::from_secs(2))
              .send()
              .await {
        Ok(response) => {
            // Parse the JSON response
            match response.json::<serde_json::Value>().await {
                Ok(json) => {
                    // Check if solution is valid based on the simplified response format
                    let is_valid = json.get("valid").and_then(|v| v.as_bool()).unwrap_or(false);
                    
                    // Log the validation result
                    tracing::info!("Sensor response time status: {}",
                        if is_valid { "OPTIMIZED" } else { "NEEDS OPTIMIZATION" }
                    );
                    
                    is_valid
                },
                Err(e) => {
                    tracing::error!(error = %e, "Failed to parse validation response");
                    false
                }
            }
        },
        Err(e) => {
            tracing::error!(error = %e, "Failed to connect to validation endpoint");
            false
        }
    }
}

async fn get_system_status(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // Create a span for tracking aquarium system health check
    let span = tracing::info_span!("aquarium_health_check");
    let _guard = span.enter();
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
    
    // Calculate response time
    let response_time = start.elapsed().as_millis();
    
    // Check challenge statuses
    let latency_solved = check_challenge_1_status().await;
    let query_solved = false; // Challenge #2
    let error_solved = std::env::var("CHALLENGE_3_SOLVED").is_ok();
    let resource_solved = false; // Challenge #4
    // Challenge #5 is already solved
    
    // Determine overall system status
    let (env_monitoring_status, species_db_status, feeding_system_status, remote_monitoring_status, overall_status) = 
        if latency_solved && query_solved && error_solved && resource_solved {
            ("online", "online", "online", "online", "operational")
        } else {
            // Set individual statuses based on challenge completion
            (
                if latency_solved { "online" } else { "degraded" },
                if query_solved { "online" } else { "degraded" },
                if error_solved { "online" } else { "degraded" },
                if resource_solved { "online" } else { "degraded" },
                "critical" // Overall status is critical until all challenges are solved
            )
        };
    
    // Log the system status check with metrics
    tracing::info!(
        system_status = %overall_status,
        response_time_ms = response_time as f64,
        "System status retrieved"
    );
    
    // Return system status
    let status = SystemStatus {
        environmental_monitoring: env_monitoring_status.to_string(),
        species_database: species_db_status.to_string(),
        feeding_system: feeding_system_status.to_string(),
        remote_monitoring: remote_monitoring_status.to_string(),
        analysis_engine: "online".to_string(), // Already solved with Challenge #5
        overall_status: overall_status.to_string(),
        last_updated: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(Json(status))
}

#[derive(Serialize)]
struct ChallengeSolution {
    code: String,
    explanation: String,
    lecture: String,
}

async fn get_current_challenge() -> impl IntoResponse {
    // Create a span for tracking system optimization status
    let span = tracing::info_span!("system_optimization_status");
    let _guard = span.enter();
    
    // This endpoint returns all challenges and their statuses
    // It can be used by the frontend to display challenge progress
    
    // Check if challenges are solved by making API calls to test endpoints
    let latency_solved = check_challenge_1_status().await;
    let query_solved = false;   // Will be true when species search is optimized
    let resource_solved = false; // Will be true when the sensor status uses a static client
    // Challenge #5 has been updated to RESTful API structure and is already solved
    // Log event for dashboard tracking - using consistent field naming
    tracing::info!("System optimization status: sensor_latency={}, database_query={}, resource_usage={}, api_structure=optimal",
        if latency_solved { "optimized" } else { "suboptimal" },
        if query_solved { "optimized" } else { "suboptimal" },
        if resource_solved { "optimized" } else { "suboptimal" }
    );
    
    // Define detailed challenge information
    let challenge_1_solution = ChallengeSolution {
        code: r#"// Replace this line:
let _config = std::fs::read_to_string("./config/tank_settings.json")
    .unwrap_or_else(|_| "{}".to_string());

// With this:
let _config = tokio::fs::read_to_string("./config/tank_settings.json")
    .await
    .unwrap_or_else(|_| "{}".to_string());
"#.to_string(),
        explanation: "This solution replaces a blocking file I/O operation with an async version that won't block the entire thread.".to_string(),
        lecture: r#"# Understanding Blocking vs. Non-Blocking I/O in Rust

## The Problem

In an async Rust application, using synchronous I/O operations like `std::fs::read_to_string()` blocks the entire thread until the operation completes. This means:

- No other tasks can run on that thread while waiting for I/O
- Overall throughput is reduced
- Response times become inconsistent

## The Solution

Replacing `std::fs` with `tokio::fs` makes the I/O operation truly asynchronous:

```rust
let content = tokio::fs::read_to_string("path/to/file").await?;
```

This allows the Tokio runtime to:

1. Suspend only the current task (not the entire thread)
2. Schedule other tasks to run while waiting
3. Resume the task when the I/O completes

## Why This Works

Under the hood, Tokio uses an event-driven architecture with an I/O event queue. When you call an async function and await it:

1. The current task is suspended and stored with a continuation
2. The I/O operation is registered with the OS
3. When the OS signals completion, Tokio wakes up the task
4. The task continues from where it left off

This pattern is essential for high-performance Rust services.
"#.to_string(),
    };
    
    // Return challenge status as JSON with detailed information
    Json(serde_json::json!({
        "challenges": [
            {
                "id": 1,
                "name": "latency-issue",
                "title": "The Sluggish Sensor",
                "description": "The environmental monitoring system is experiencing severe delays, preventing timely readings of tank conditions. Every second counts when maintaining delicate ecosystems!",
                "hint": "Look for blocking operations in the tank readings function. In async Rust, using std::fs blocks the entire thread. Is there an async alternative?",
                "service": "aqua-monitor",
                "file": "src/main.rs",
                "function": "get_tank_readings",
                "status": if latency_solved { "solved" } else { "pending" },
                "solution": challenge_1_solution
            },
            {
                "id": 2,
                "name": "query-optimization",
                "title": "The Query Conundrum",
                "description": "The species database is responding slowly to searches, making it difficult to quickly identify specimens and their requirements.",
                "hint": "Check how the database is searching for species names. Are the queries using indexes? Could you make the search case-insensitive too?",
                "service": "species-hub",
                "file": "src/main.rs",
                "function": "get_species",
                "status": if query_solved { "solved" } else { "pending" },
                "solution": ChallengeSolution {
                    code: "// Solution code will be provided".to_string(),
                    explanation: "Database indexing and query optimization solution".to_string(),
                    lecture: "Lecture on database indexing and SQL optimization".to_string()
                }
            },
            {
                "id": 3,
                "name": "error-handling",
                "title": "The Fragile Feeder",
                "description": "The feeding schedule system crashes when encountering unexpected data, disrupting critical feeding operations for multiple tanks.",
                "hint": "Instead of panicking on errors, implement proper Rust error handling with the Result type and ? operator.",
                "service": "species-hub",
                "file": "src/main.rs",
                "function": "get_feeding_schedule",
                "status": if std::env::var("CHALLENGE_3_SOLVED").is_ok() { "solved" } else { "pending" },
                "solution": ChallengeSolution {
                    code: "// Solution code will be provided".to_string(),
                    explanation: "Proper error handling solution".to_string(),
                    lecture: "Lecture on Rust error handling".to_string()
                }
            },
            {
                "id": 4,
                "name": "resource-leak",
                "title": "The Leaky Connection",
                "description": "The sensor status API is creating a new HTTP client for every request, causing excessive resource usage and potential memory leaks.",
                "hint": "Create a shared, static client that can be reused across requests instead of creating a new one each time.",
                "service": "aqua-monitor",
                "file": "src/main.rs",
                "function": "get_sensor_status",
                "status": if resource_solved { "solved" } else { "pending" },
                "solution": ChallengeSolution {
                    code: "// Solution code will be provided".to_string(),
                    explanation: "Resource management solution".to_string(),
                    lecture: "Lecture on resource management in Rust".to_string()
                }
            },
            {
                "id": 5,
                "name": "shared-state-mutex",
                "title": "Safe Shared State",
                "description": "You need to maintain a shared counter or cache across requests, but naive approaches can cause data races or panics. Use Rust's async-safe Mutex to implement correct shared state.",
                "hint": "Use Arc<Mutex<T>> or tokio::sync::Mutex for shared mutable state. Beware of holding locks across .await points!",
                "service": "aqua-brain",
                "file": "src/main.rs",
                "function": "shared_state_example",
                "status": "pending",
                "solution": ChallengeSolution {
                    code: "// Example using Arc<tokio::sync::Mutex<T>>".to_string(),
                    explanation: "This solution demonstrates how to safely share and mutate state across async requests using a lock.".to_string(),
                    lecture: "Lecture on Mutex, Arc, and shared state in async Rust".to_string()
                }
            }
        ],
        "total": 5,
        "solved": (if latency_solved { 1 } else { 0 }) + 
                 (if query_solved { 1 } else { 0 }) + 
                 (if std::env::var("CHALLENGE_3_SOLVED").is_ok() { 1 } else { 0 }) +
                 (if resource_solved { 1 } else { 0 }) + 
                 1 // Challenge #5 is now always solved
    }))
}

// Dedicated endpoint for testing Challenge #1
async fn test_challenge_1() -> impl IntoResponse {
    // Create a span for tracking sensor latency diagnostics
    let span = tracing::info_span!("sensor_latency_diagnostic");
    let _guard = span.enter();
    
    tracing::info!("Sensor response time diagnostic requested");
    
    // Check if the challenge is solved by validating the current implementation
    let start = std::time::Instant::now();
    let is_solved = check_challenge_1_status().await;
    let test_duration = start.elapsed().as_millis();
    
    // Create a streamlined response with only essential information
    let response = serde_json::json!({
        "id": 1,
        "name": "The Sluggish Sensor",
        "solved": is_solved,
        "message": if is_solved {
            "Challenge solved! You've successfully implemented the async file I/O solution."
        } else {
            "Replace std::fs::read_to_string with tokio::fs::read_to_string and add .await to make the file I/O operation async."
        },
        "test_time_ms": test_duration
    });
    
    tracing::info!(
        challenge_id = 1,
        challenge_name = "latency-issue",
        test_duration_ms = test_duration,
        challenge_solved = is_solved,
        "Challenge #1 test completed"
    );
    
    Json(response)
}

// Function to test a submitted solution for Challenge #1
async fn test_challenge_1_solution(
    State(_state): State<AppState>,
    Json(payload): Json<Challenge1SolutionRequest>,
) -> impl IntoResponse {
    // Create a span for tracking sensor optimization implementation
    let span = tracing::info_span!("sensor_optimization_implementation");
    let _guard = span.enter();
    
    tracing::info!("Sensor optimization code submitted for implementation");
    
    // Validate the solution code
    let is_valid_solution = payload.code.contains("tokio::fs::read_to_string") && 
                           payload.code.contains(".await");
    
    tracing::info!(
        solution.valid = is_valid_solution,
        solution.contains_tokio_fs = payload.code.contains("tokio::fs::read_to_string"),
        solution.contains_await = payload.code.contains(".await"),
        "Solution validation completed"
    );
    
    // Create detailed response with feedback
    let response = serde_json::json!({
        "challenge": {
            "id": 1,
            "name": "latency-issue",
            "title": "The Sluggish Sensor",
        },
        "test_results": {
            "success": true,
            "valid_solution": is_valid_solution,
            "message": if is_valid_solution {
                "Your solution looks correct! Now edit the actual code in the aqua-monitor service to implement it."
            } else {
                "Your solution doesn't appear to use tokio::fs::read_to_string with .await. Make sure you're replacing the blocking I/O with an async version."
            },
            "details": "Replace std::fs::read_to_string with tokio::fs::read_to_string and add .await to make the file I/O operation async."
        }
    });
    
    tracing::info!("Sensor optimization validation completed, status: {}", 
        if is_valid_solution { "SUCCESSFUL" } else { "NEEDS REVISION" }
    );
    
    Json(response)
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
    // Create a span for tracking multi-tank environmental analysis
    let span = tracing::info_span!("multi_tank_analysis");
    let _guard = span.enter();
    
    // Add request ID for correlation and timing
    let request_id = uuid::Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();
    
    tracing::info!(
        request_id = %request_id,
        operation = "multi_tank_analysis",
        tanks_requested = "all",
        "Starting multi-tank environmental analysis"
    );
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
        
    // Log timing information on completion
    let elapsed = start_time.elapsed().as_millis() as f64;
    tracing::info!(
        request_id = %request_id,
        operation = "multi_tank_analysis",
        tanks_analyzed = results.len(),
        analysis_duration_ms = elapsed,
        operation_status = "success",
        "Multi-tank environmental analysis completed"
    );
    
    Json(results)
}

// Handler for single tank analysis by ID
async fn get_tank_analysis_by_id(
    State(_state): State<AppState>,
    Path(tank_id): Path<String>,
    Query(params): Query<AnalysisParams>,
) -> impl IntoResponse {
    // Create a span for tracking single tank environmental analysis
    let span = tracing::info_span!("single_tank_analysis");
    let _guard = span.enter();
    
    // Add request ID for correlation and timing
    let request_id = uuid::Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();
    
    tracing::info!(
        request_id = %request_id,
        tank_id = %tank_id,
        operation = "single_tank_analysis",
        "Starting tank environmental analysis"
    );
    // Override tank_id from path parameter
    let mut tank_params = params;
    // Clone tank_id directly in the assignment to keep the original for logging
    tank_params.tank_id = Some(tank_id.clone());
    
    // Get single tank analysis
    let result = get_analysis_result(tank_params);
    
    // Log timing information on completion
    let elapsed = start_time.elapsed().as_millis() as f64;
    tracing::info!(
        request_id = %request_id,
        tank_id = %tank_id,
        analysis_duration_ms = elapsed,
        overall_health = %result.overall_health,
        operation_status = "success",
        "Tank environmental analysis completed"
    );
    
    Json(result)
}

// Helper function to generate analysis result (extracted from analyze_tank_conditions)
fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // No timing needed here as we're not measuring performance
    // Get tank_id or default to Tank-A1
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string());
    
    // Log the analysis operation with structured fields
    tracing::debug!(
        tank_id = %tank_id,
        species_id = params.species_id,
        analysis_type = "environmental",
        operation = "tank_analysis",
        "Analyzing tank environmental conditions"
    );
    
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

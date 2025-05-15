use shuttle_axum::axum::extract::{Path, Query, State};
use shuttle_axum::axum::http::StatusCode;
use shuttle_axum::axum::response::IntoResponse;
use shuttle_axum::axum::routing::get;
use shuttle_axum::axum::Json;
use shuttle_axum::axum::Router;
// CORS removed - managed by frontend
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

// Empty AppState following KISS principle - no direct service-to-service communication
#[derive(Clone)]
struct AppState {}

// SystemStatus removed - following KISS principle
// Each service should report its own status
// Frontend is responsible for aggregating status information

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
    // Initialize state - no clients needed following our KISS architecture
    let state = AppState {};
    
    // Build router
    let router = Router::new()
        // No system status endpoint - following KISS principle, the frontend should
        // call each service directly and compute the overall status
        .route("/api/analysis/tanks", get(get_all_tank_analysis))
        .route("/api/analysis/tanks/:tank_id", get(get_tank_analysis_by_id))
        .route("/api/challenges/current", get(get_current_challenge))
        .route("/api/challenges/test/1", get(test_challenge_1))
        // Challenge solution validation should be in the service where the implementation resides
        // For Challenge #1, validation is done in the aqua-monitor service
        .route("/api/health", get(health_check))
        .with_state(state);
    
    Ok(router.into())
}


#[derive(Serialize)]
struct ChallengeSolution {
    code: String,
    explanation: String,
    lecture: String,
}

async fn get_current_challenge() -> impl IntoResponse {
    // Create a span for tracking challenge metadata requests
    let span = tracing::info_span!("challenge_metadata_request");
    let _guard = span.enter();
    let request_id = uuid::Uuid::new_v4().to_string();
    
    tracing::info!(
        request_id = %request_id,
        operation = "get_challenge_metadata",
        "Providing challenge metadata"
    );
    
    // Define detailed challenge information
    let challenge_1_solution = ChallengeSolution {
        code: r#"// Before: Blocking I/O
// let config = std::fs::read_to_string("./config/tank_settings.json")
//     .unwrap_or_else(|_| "{}".to_string());
// Also remove the artificial delay
// std::thread::sleep(std::time::Duration::from_millis(100));

// After: Async I/O
let config = tokio::fs::read_to_string("./config/tank_settings.json").await
    .unwrap_or_else(|_| "{}".to_string());
"#.to_string(),
        explanation: "This solution replaces a blocking file I/O operation with an async version that won't block the entire thread, significantly improving response times and system throughput.".to_string(),
        lecture: r#"# Understanding Blocking vs. Non-Blocking I/O in Rust

## The Problem with Blocking I/O

In an async Rust application, using synchronous I/O operations like `std::fs::read_to_string()` blocks the entire thread until the operation completes. This means:

- No other tasks can run on that thread while waiting for I/O
- Overall throughput is reduced
- Response times become inconsistent
- System scalability is limited

## The Async I/O Solution

Replacing `std::fs` with `tokio::fs` makes the I/O operation truly asynchronous:

```rust
// Before: Blocking I/O
// let config = std::fs::read_to_string("./config/tank_settings.json")
//     .unwrap_or_else(|_| "{}".to_string());
// Also remove the artificial delay
// std::thread::sleep(std::time::Duration::from_millis(100));

// After: Async I/O
let config = tokio::fs::read_to_string("./config/tank_settings.json").await
    .unwrap_or_else(|_| "{}".to_string());
```

## Performance Benefits

Async I/O provides several key benefits:

1. **Concurrency**: Multiple I/O operations can be in progress simultaneously
2. **Throughput**: The system can handle more requests per second
3. **Responsiveness**: Critical operations don't get blocked by slow I/O
4. **Resource Efficiency**: Thread resources aren't wasted waiting for I/O

## How Tokio's Async I/O Works

Under the hood, Tokio uses an event-driven architecture with an I/O event queue:

1. When you call an async function and `.await` it, the current task is suspended
2. The I/O operation is registered with the OS's async I/O facilities
3. The thread is free to process other tasks while waiting
4. When the OS signals completion, Tokio wakes up the task
5. The task continues from where it left off

This cooperative multitasking model is the foundation of modern high-performance services.

## Best Practices

- Use async I/O for all potentially slow operations (file, network, etc.)
- Keep CPU-intensive work off the async runtime when possible
- Remember that `.await` points are where your function can be suspended
- Use proper error handling with async operations
"#.to_string(),
    };
    
    // Return challenge metadata as JSON with detailed information including validation endpoints
    Json(serde_json::json!({
        "challenges": [
            {
                "id": 1,
                "name": "latency-issue",
                "title": "The Sluggish Sensor",
                "description": "The environmental monitoring system is experiencing severe delays, preventing timely readings of tank conditions. The file I/O operations seem particularly slow, with response times far exceeding what should be expected even for standard disk operations. Every second counts when maintaining delicate ecosystems!",
                "hint": "Look for blocking operations in the get_tank_readings function. In async Rust, using std::fs::read_to_string blocks the entire thread. Also, check if there are any artificial delays that might be contributing to the slowdown. The solution involves both using the right async I/O method and ensuring there are no unnecessary waits.",
                "service": "aqua-monitor",
                "file": "src/main.rs",
                "function": "get_tank_readings",
                "status": "degraded", // Frontend must query the service directly for status
                "validation_endpoint": {
                    "service": "aqua-monitor",
                    "url": "/api/challenges/1/validate",
                    "method": "GET"
                },
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
                "status": "degraded", // Frontend must query the service directly for status
                "validation_endpoint": {
                    "service": "species-hub",
                    "url": "/api/species/validate-solution", // This will need to be implemented in species-hub
                    "method": "GET"
                },
                "solution": ChallengeSolution {
                    code: r#"// Before: Inefficient LIKE query with case-sensitivity
// sqlx::query("SELECT * FROM species WHERE name LIKE $1")
//     .bind(format!("%{}%", name))

// After: Optimized ILIKE query with trigram index
sqlx::query("SELECT * FROM species WHERE name ILIKE $1")
    .bind(format!("%{}%", name))
"#.to_string(),
                    explanation: "This solution replaces case-sensitive LIKE queries with case-insensitive ILIKE queries that can utilize PostgreSQL's trigram indexes, dramatically improving search performance.".to_string(),
                    lecture: r#"# Database Query Optimization with Indexes

## The Problem with Non-Indexed LIKE Queries

When searching text fields with LIKE operators in SQL databases, performance suffers dramatically without proper indexing:

- Each search requires a full table scan, examining every row
- Query time grows linearly with table size
- Case-sensitive searches miss potential matches
- System throughput decreases under load

## The Optimized Solution

Two key improvements make our queries fast and user-friendly:

```sql
-- Before: Inefficient LIKE query with case-sensitivity
-- SELECT * FROM species WHERE name LIKE '%search_term%'

-- After: Optimized ILIKE query (works with trigram index)
SELECT * FROM species WHERE name ILIKE '%search_term%'
```

## How PostgreSQL Trigram Indexes Work

Trigram indexes break text into 3-character sequences, enabling efficient pattern matching:

1. The text "Clownfish" produces trigrams: "clo", "low", "own", "wnf", "nfi", "fis", "ish"
2. Searches for similar patterns can use the index rather than scanning the table
3. ILIKE queries automatically benefit from trigram indexes when available

## Performance Benefits

- **Search Speed**: Queries can be 10-100x faster on large tables
- **Case Insensitivity**: Finding "clownfish" when users search for "Clownfish"
- **Scalability**: Performance stays consistent as the database grows
- **User Experience**: Faster, more intuitive search results

## Best Practices

- Use `CREATE INDEX idx_name ON table USING gin (column gin_trgm_ops)` for trigram indexes
- Consider partial indexes if you only search a subset of data
- Analyze query plans with EXPLAIN ANALYZE to verify index usage
- Monitor index size and rebuild periodically for optimal performance
"#.to_string()
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
                "status": "degraded", // Frontend must query the service directly for status
                "validation_endpoint": {
                    "service": "species-hub",
                    "url": "/api/feeding/validate-solution", // This will need to be implemented in species-hub
                    "method": "GET"
                },
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
                "status": "degraded", // Frontend must query the service directly for status
                "validation_endpoint": {
                    "service": "aqua-monitor",
                    "url": "/api/sensors/validate-solution", // This will need to be implemented in aqua-monitor
                    "method": "GET"
                },
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
                "status": "degraded", // Frontend must query the service directly for status
                "validation_endpoint": {
                    "service": "aqua-brain",
                    "url": "/api/shared-state/validate-solution", // Need to implement this endpoint in aqua-brain
                    "method": "GET"
                },
                "solution": ChallengeSolution {
                    code: "// Example using Arc<tokio::sync::Mutex<T>>".to_string(),
                    explanation: "This solution demonstrates how to safely share and mutate state across async requests using a lock.".to_string(),
                    lecture: "Lecture on Mutex, Arc, and shared state in async Rust".to_string()
                }
            }
        ],
        "total": 5,
        "solved": 0, // Following KISS, the frontend will determine this by calling each service's validation endpoint
    }))
}

// Dedicated endpoint for testing Challenge #1
async fn test_challenge_1() -> impl IntoResponse {
    // Create a span for tracking sensor latency diagnostics
    let span = tracing::info_span!("sensor_latency_diagnostic");
    let _guard = span.enter();
    
    tracing::info!("Sensor response time diagnostic requested");
    
    // Create a response with only the information needed for the frontend
    let response = serde_json::json!({
        "id": 1,
        "name": "The Sluggish Sensor",
        "message": "For validation, please call the aqua-monitor service at /api/challenges/1/validate",
        "hint": "Replace std::fs::read_to_string with tokio::fs::read_to_string and add .await to make the file I/O operation async.",
        "system_component": {
            "name": "Analysis Engine",
            "status": "normal",
            "description": "Analysis engine operating normally"
        }
    });
    
    tracing::info!(
        challenge_id = 1,
        challenge_name = "latency-issue",
        "Challenge #1 test endpoint called - redirecting to service-specific validation"
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

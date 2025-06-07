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
use std::fs;

// Function to load lecture content from markdown files
fn load_lecture(challenge_number: usize) -> String {
    let lecture_path = format!("src/lectures/challenge{}.md", challenge_number);
    match fs::read_to_string(&lecture_path) {
        Ok(content) => content,
        Err(e) => {
            tracing::warn!(
                error = %e,
                path = %lecture_path,
                "Failed to load lecture content from file"
            );
            format!("# Lecture for Challenge #{} \n\nLecture content could not be loaded.", challenge_number)
        }
    }
}

// Function to load solution content from markdown files
fn load_solution(challenge_number: usize) -> (String, String) {
    let solution_path = format!("src/lectures/challenge{}_solution.md", challenge_number);
    match fs::read_to_string(&solution_path) {
        Ok(content) => {
            // Extract code and explanation from the content
            // The format is expected to be:
            // ```rust
            // code here
            // ```
            // explanation text
            
            let code_start = content.find("```").unwrap_or(0);
            let code_end = content[code_start + 3..].find("```").unwrap_or(content.len());
            
            // Extract just the code without the markdown code fence
            let code_with_lang = &content[code_start..code_start + 3 + code_end + 3];
            let code_content = code_with_lang
                .lines()
                .skip(1) // Skip the ```rust line
                .take(code_with_lang.lines().count() - 2) // Skip the closing ```
                .collect::<Vec<&str>>()
                .join("\n");
            
            // Extract the explanation (everything after the code block)
            let explanation_start = code_start + 3 + code_end + 3;
            let explanation = if explanation_start < content.len() {
                content[explanation_start..].trim().to_string()
            } else {
                "Solution explanation unavailable".to_string()
            };
            
            (code_content, explanation)
        },
        Err(e) => {
            tracing::warn!(
                error = %e,
                path = %solution_path,
                "Failed to load solution content from file"
            );
            (
                format!("// Solution code for Challenge #{} unavailable", challenge_number),
                format!("Solution explanation for Challenge #{} unavailable", challenge_number)
            )
        }
    }
}

// Import challenges module
mod challenges;

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
struct AppState {}

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

#[derive(Serialize, Deserialize, Clone)]
struct ChallengeSolution {
    code: String,
    explanation: String,
    lecture: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EndpointInfo {
    service: String,
    url: String,
    method: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Challenge {
    id: usize,
    name: String,
    title: String,
    description: String,
    hint: String,
    service: String,
    file: String,
    function: String,
    status: String,
    validation_endpoint: EndpointInfo,
    solution: ChallengeSolution,
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
    // Load challenge solutions from markdown files
    let (code1, explanation1) = load_solution(1);
    let (code2, explanation2) = load_solution(2);
    let (code3, explanation3) = load_solution(3);
    let (code4, explanation4) = load_solution(4);

    // Create solution objects with the loaded content
    let challenge_1_solution = ChallengeSolution {
        code: code1,
        explanation: explanation1,
        lecture: load_lecture(1),
    };
    
    let challenge_2_solution = ChallengeSolution {
        code: code2,
        explanation: explanation2,
        lecture: load_lecture(2),
    };
    
    let challenge_3_solution = ChallengeSolution {
        code: code3,
        explanation: explanation3,
        lecture: load_lecture(3),
    };
    
    let challenge_4_solution = ChallengeSolution {
        code: code4,
        explanation: explanation4,
        lecture: load_lecture(4),
    };
    
    // Define challenge metadata for the current ongoing challenges
    let challenges = vec![
        Challenge {
            id: 1,
            name: "async-io".to_string(),
            title: "The Blocking Bottleneck".to_string(),
            description: "The tank parameter validation process is using blocking I/O operations, causing performance issues during peak usage. This is causing the monitoring system to miss critical water quality changes.".to_string(),
            hint: "Look for blocking I/O operations in the validate_tank_parameters function. You'll need to replace standard library functions with their asynchronous equivalents from the tokio crate. Pay special attention to file operations and sleep functions.".to_string(),
            service: "aqua-monitor".to_string(),
            file: "src/challenges.rs".to_string(),
            function: "validate_tank_parameters".to_string(),
            status: "degraded".to_string(),
            validation_endpoint: EndpointInfo {
                service: "aqua-monitor".to_string(),
                url: "/api/challenges/1/validate".to_string(),
                method: "GET".to_string()
            },
            solution: challenge_1_solution
        },
        Challenge {
            id: 2,
            name: "database-optimization".to_string(),
            title: "The Slow Query".to_string(),
            description: "The species search functionality is extremely slow when users search for partial names. Database queries are taking too long, especially for text searches.".to_string(),
            hint: "The issue is with how text search is being performed in the database. Look at how case-sensitivity is handled in the SQL queries. PostgreSQL offers operators for more efficient case-insensitive pattern matching that would improve search performance.".to_string(),
            service: "species-hub".to_string(),
            file: "src/challenges.rs".to_string(),
            function: "search_species".to_string(),
            status: "degraded".to_string(),
            validation_endpoint: EndpointInfo {
                service: "species-hub".to_string(),
                url: "/api/challenges/2/validate".to_string(),
                method: "GET".to_string()
            },
            solution: challenge_2_solution
        },
        Challenge {
            id: 3,
            name: "memory-optimization".to_string(),
            title: "The Memory Hog".to_string(),
            description: "The analysis engine is using excessive memory, particularly when calculating status reports for multiple tanks. The issue seems to be with how strings are handled.".to_string(),
            hint: "Examine how strings are created and returned in the tank analysis functions. Reduce memory usage by either: 1) using static string references (&'static str) for fixed values, or 2) implementing string interning with the internment crate to deduplicate repeated strings.".to_string(),
            service: "aqua-brain".to_string(),
            file: "src/challenges.rs".to_string(),
            function: "analyze_tank_conditions".to_string(),
            status: "degraded".to_string(),
            validation_endpoint: EndpointInfo {
                service: "aqua-brain".to_string(),
                url: "/api/challenges/3/validate".to_string(),
                method: "GET".to_string()
            },
            solution: challenge_3_solution
        },
        Challenge {
            id: 4,
            name: "resource-leak".to_string(),
            title: "The Leaky Connection".to_string(),
            description: "The sensor status API is creating a new HTTP client for every request, causing excessive resource usage and potential memory leaks.".to_string(),
            hint: "Look for where a new HTTP client is being created for each request in the get_sensor_status function. Creating HTTP clients is expensive! Each client maintains connection pools, TLS configurations, and system resources. To fix this, create a shared, static HTTP client that can be reused across all requests. Consider using lazy_static or once_cell to create a properly initialized static client.".to_string(),
            service: "aqua-monitor".to_string(),
            file: "src/challenges.rs".to_string(),
            function: "get_sensor_status".to_string(),
            status: "degraded".to_string(),
            validation_endpoint: EndpointInfo {
                service: "aqua-monitor".to_string(),
                url: "/api/challenges/4/validate".to_string(),
                method: "GET".to_string()
            },
            solution: challenge_4_solution
        }
    ];
    
    // Return challenge metadata as JSON
    Json(serde_json::json!({
        "challenges": challenges,
        "total": challenges.len(),
        "solved": 0,
    }))
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // Initialize state - no clients needed following our KISS architecture
    let state = AppState {};
    
    // Build router
    let router = Router::new()
        .route("/api/analysis/tanks", get(get_all_tank_analysis))
        .route("/api/analysis/tanks/:tank_id", get(get_tank_analysis_by_id))
        .route("/api/challenges/current", get(get_current_challenge))
        .route("/api/challenges/test/1", get(challenges::test_challenge_1))
        .route("/api/challenges/3/validate", get(validate_memory_optimization))
        // Challenge solution validation should be in the service where the implementation resides
        // For Challenge #1, validation is done in the aqua-monitor service
        .route("/api/health", get(health_check))
        .with_state(state);
    
    Ok(router.into())
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
            let full_analysis = challenges::get_analysis_result(tank_params);
            
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
    let result = challenges::get_analysis_result(tank_params);
    
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

/// Validates the implementation of Challenge #3: Memory Optimization
async fn validate_memory_optimization(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("Starting validation for Challenge #3: Memory Optimization");
    
    use serde_json::json;
    use std::fs;

    // Create a request ID for correlation in logs
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Extract just the challenge code section using the challenge markers
    let source_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("src/challenges.rs");
    
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
                    "name": "Analysis Engine",
                    "description": "Analysis engine is experiencing high memory usage",
                    "status": "degraded"
                }
            })));
        }
    };
    
    // Find the challenge section boundaries
    let challenge_start = source_code.find("// ⚠️ CHALLENGE #3: MEMORY OPTIMIZATION ⚠️");
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
                "name": "Analysis Engine",
                "description": "Analysis engine is experiencing high memory usage",
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
    
    // Count the number of uncommented .to_string() calls in the challenge code
    let to_string_count = challenge_code.lines()
        .filter(|line| !line.trim().starts_with("//"))
        .filter(|line| line.contains(".to_string()"))
        .count();
    
    // Check for the use of optimized string handling (either &str or internment)
    let uses_str_type = is_uncommented("&str") || is_uncommented("&'static str");
    let uses_interning = is_uncommented("Intern::") || is_uncommented("internment::");
    
    // Log what we're finding in the challenge code
    tracing::info!(
        request_id = %request_id,
        to_string_count = to_string_count,
        uses_str_type = uses_str_type,
        uses_interning = uses_interning,
        "Challenge code check results"
    );
    
    // The challenge is completed if the number of .to_string() calls is significantly reduced
    // and either &str or interning is used
    let is_valid = to_string_count < 10 && (uses_str_type || uses_interning);
    
    tracing::info!(
        request_id = %request_id,
        solution_valid = is_valid,
        "Challenge #3 validation completed"
    );
    
    // Build a standardized response following the same format as other challenges
    let response = json!({
        "valid": is_valid,
        "message": if is_valid {
            "Solution correctly implemented! Memory usage is now optimized."
        } else {
            "Solution validation failed. Please optimize memory usage by using either static string references or string interning instead of creating new String objects."
        },
        "system_component": {
            "name": "Analysis Engine",
            "description": if is_valid {
                "Analysis engine memory usage is now optimized"
            } else {
                "Analysis engine is experiencing high memory usage"
            },
            "status": if is_valid { "normal" } else { "degraded" }
        }
    });
    
    (StatusCode::OK, Json(response))
}

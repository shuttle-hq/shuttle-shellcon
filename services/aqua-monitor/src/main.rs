use shuttle_axum::axum::{
    extract::{Path, State},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::time::Duration;
use std::sync::atomic::AtomicUsize;
use tracing;
use thiserror::Error;

// Custom Error Type for aqua-monitor service
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Sensor error: {0}")]
    SensorError(String),
    
    #[error("Tank not found: {0}")]
    TankNotFound(String),
    
    #[error("External service error: {0}")]
    ExternalService(#[from] reqwest::Error),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Implement IntoResponse for our custom error type
impl IntoResponse for ApiError {
    fn into_response(self) -> shuttle_axum::axum::response::Response {
        let (status, error_message) = match &self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::SensorError(msg) => (StatusCode::SERVICE_UNAVAILABLE, format!("Sensor error: {}", msg)),
            ApiError::TankNotFound(id) => (StatusCode::NOT_FOUND, format!("Tank not found: {}", id)),
            ApiError::ExternalService(_) => (StatusCode::BAD_GATEWAY, "External service error".to_string()),
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
            "timestamp": chrono::Utc::now()
        }))).into_response()
    }
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct TankReading {
    id: i32,
    tank_id: String,
    temperature: f64,
    ph: f64,
    oxygen_level: f64,
    salinity: f64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Initialize database with logging and proper error handling
    tracing::info!("Running database migrations for aqua-monitor...");
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        tracing::error!(error = %e, "Database migration failed for aqua-monitor");
        return Err(anyhow::anyhow!("Database migration failed: {e}").into());
    }
    tracing::info!("Database migrations completed successfully for aqua-monitor.");
    
    // Initialize state
    let state = AppState { pool };
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);
        
    // Build router with CORS
    let router = Router::new()
        .route("/api/tanks", get(get_all_tanks))
        .route("/api/tanks/{id}/readings", get(get_tank_readings))
        .route("/api/sensors/status", get(get_sensor_status))
        .route("/api/health", get(health_check))
        .with_state(state)
        .layer(cors);
    

    Ok(router.into())
}

// Returns a list of all unique tank IDs
async fn get_all_tanks(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let rows = sqlx::query("SELECT DISTINCT tank_id FROM tank_readings")
        .fetch_all(&state.pool)
        .await?;
    let tank_ids: Vec<String> = rows.into_iter()
        .map(|row| row.get::<String, _>("tank_id"))
        .collect();
    Ok(Json(tank_ids))
}

// CHALLENGE #1: Fix the blocking operation in this function
// This function has a blocking operation that's causing high latency
async fn get_tank_readings(
    Path(tank_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    
    // Start timing the request
    let start = std::time::Instant::now();
    
    // PROBLEM: Simple blocking file I/O - just read a configuration file
    // This single line is the entire issue
    let _config = std::fs::read_to_string("./config/tank_settings.json")
        .unwrap_or_else(|_| "{}".to_string());

    // First, check if the tank exists
    if tank_id.is_empty() {
        return Err(ApiError::TankNotFound("empty tank ID".to_string()));
    }
    
    // Async database query
    let readings = sqlx::query_as::<_, TankReading>(
        "SELECT * FROM tank_readings WHERE tank_id = $1 ORDER BY timestamp DESC LIMIT 10"
    )
    .bind(&tank_id)
    .fetch_all(&state.pool)
    .await
    // Propagate database errors
    .map_err(ApiError::Database)?;
    
    // If no readings found, you could return a specialized error
    if readings.is_empty() {
        // For this example, we'll just return empty results
        // But in a real app, you might want to return a specific error:
        // return Err(ApiError::TankNotFound(format!("No readings for tank {}", tank_id)));
    }
    
    // Calculate the actual request duration
    let elapsed = start.elapsed().as_millis() as f64;
    
    // Emit challenge status based on actual latency
    if elapsed < 100.0 {
        tracing::info!(
            event.challenge_solved = "latency",
            challenge.id = 1,
            challenge.status = "solved",
            "Challenge #1 Solved: Latency optimized!"
        );
    }
    
    // Custom metric showing request duration
    tracing::info!(
        histogram.request_duration_ms = elapsed,
        api.endpoint = "tank_readings",
        challenge.current_latency = elapsed,
        tank.id = %tank_id,
        db.rows_returned = readings.len(),
        "Tank readings retrieved"
    );
    
    Ok(Json(readings))
}

// CHALLENGE #4: Fix the resource leak in this function
// This function creates a new client for every request

// SOLUTION: Use a static client with once_cell
// For the challenge: Track how many clients have been created
static CLIENT_COUNT: AtomicUsize = AtomicUsize::new(0);

// Uncomment to solve the challenge:
// static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
//     reqwest::Client::builder()
//         .timeout(Duration::from_secs(10))
//         .build()
//         .expect("Failed to create HTTP client")
// });

async fn get_sensor_status(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    let _start = std::time::Instant::now();
    
    // ⚠ FIX NEEDED HERE ⚠
    // This intentionally creates a new client for every request
    // causing resource leakage
    let client = reqwest::Client::new();
    
    // Increment and track client count
    let clients_created = CLIENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
    
    // Custom metric tracking active connections - using real value
    tracing::info!(
        counter.active_connections = 1,
        counter.total_clients_created = clients_created as i64,
        "Created new sensor connection"
    );
    
    // Emit challenge status (solved if using static client)
    if false { // Change to: if std::option_env!("USING_STATIC_CLIENT").is_some() {
        tracing::info!(
            event.challenge_solved = "resource_leak",
            challenge.id = 4,
            challenge.status = "solved",
            "Challenge #4 Solved: Resource leak fixed!"
        );
    }
    
    // Simulate external sensor API call
    let response = client.get("https://api.example.com/sensors")
        .timeout(Duration::from_secs(2))
        .send()
        .await;
    
    match response {
        Ok(res) => {
            if res.status().is_success() {
                // Return mock data since this is an example
                return Json(serde_json::json!({
                    "status": "online",
                    "active_sensors": 24,
                    "last_updated": chrono::Utc::now()
                })).into_response();
            } else {
                return (StatusCode::BAD_GATEWAY, "Sensor API error".to_string()).into_response();
            }
        }
        Err(_) => {
            // In a real app, would return actual sensor data
            return Json(serde_json::json!({
                "status": "online",
                "active_sensors": 24,
                "last_updated": chrono::Utc::now()
            })).into_response();
        }
    }
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// SOLUTION FOR CHALLENGE #1
// Replace the blocking code with async calls:
/*async fn get_tank_readings(
    Path(tank_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Proper async database query
    let readings = sqlx::query_as::<_, TankReading>(
        "SELECT * FROM tank_readings WHERE tank_id = $1 ORDER BY timestamp DESC LIMIT 10"
    )
    .bind(&tank_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();
    
    // Track request duration with tracing
    tracing::info!(
        histogram.request_duration_ms = 10.0,  // Much faster now!
        api.endpoint = "tank_readings",
        "Tank readings retrieved asynchronously"
    );
    
    Json(readings)
}*/

// SOLUTION FOR CHALLENGE #4
// Use a shared client with once_cell:
/*use once_cell::sync::Lazy;

// Shared client for all requests
static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client")
});

async fn get_sensor_status(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Use the shared client instead of creating a new one
    let response = CLIENT.get("https://api.example.com/sensors")
        .timeout(Duration::from_secs(2))
        .send()
        .await;
    
    // Rest of the function remains the same
    // ...
}*/

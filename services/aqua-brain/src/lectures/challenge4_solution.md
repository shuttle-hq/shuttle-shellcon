```rust
// Before: Creating a new client for every request
// This causes resource leaks and excessive memory usage
pub async fn get_sensor_status(State(_state): State<AppState>) -> impl IntoResponse {
    // Create a span for sensor status check with request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let span = tracing::info_span!(
        "tank_sensor_status_check",
        request_id = %request_id
    );
    let _guard = span.enter();

    // BAD: Creating a new client for every request
    // This causes memory and resource leaks
    let client = reqwest::Client::new();
    
    // Log metrics about connection creation
    tracing::info!(
        request_id = %request_id,
        "Created new HTTP client for request"
    );
    
    // Rest of the function remains the same
    // but creates a new client every time
}

// After optimization: Using Axum's AppState pattern

// In main.rs where we build the application:

// First, define an AppState struct that includes the HTTP client
pub struct AppState {
    client: reqwest::Client,
    // ... other state fields as needed
}

// Then, in your main function or app setup:
let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()
    .expect("Failed to build HTTP client");

// Add the client to your app state
let state = AppState { client };

// Use the state when building your router
let app = Router::new()
    .route("/api/sensor-status", get(get_sensor_status))
    // ... other routes
    .with_state(state);

// And in your handler function:
pub async fn get_sensor_status(State(state): State<AppState>) -> impl IntoResponse {
    // Create a span for sensor status check with request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let span = tracing::info_span!(
        "tank_sensor_status_check",
        request_id = %request_id
    );
    let _guard = span.enter();

    // GOOD: Use the client from the app state
    // No new client is created here
    let client = &state.client;
    
    // Log metrics about using the shared client
    tracing::info!(
        request_id = %request_id,
        "Using shared HTTP client from app state"
    );
    
    // Rest of the function remains the same
    // but now uses the shared client from app state
}
```

This solution addresses the resource leak by using Axum's application state pattern to share a single HTTP client across all requests. The key optimizations include:

1. Creating the HTTP client once during application startup
2. Storing the client in the application state (AppState struct)
3. Accessing the client through the state in request handlers
4. Setting appropriate timeouts and configuration on the shared client

HTTP clients are resource-intensive objects that maintain connection pools, TLS configurations, and DNS caches. Creating a new one for each request wastes these resources and can cause memory leaks and performance degradation in high-traffic services. By using a static client, we ensure that these resources are properly managed and reused.

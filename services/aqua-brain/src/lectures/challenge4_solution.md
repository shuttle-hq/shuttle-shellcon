# Challenge 4 Solution: Resource Leak Prevention

## Problem: Creating a new HTTP client for every request

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
```

## Solution 1: Using Axum's Application State Pattern (Recommended)

This is the most idiomatic approach for Axum applications, providing clear dependency injection and making testing easier.

```rust
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

## Solution 2: Using Static HTTP Client with once_cell or LazyLock

This approach is useful when you need a global client accessible from multiple contexts.

```rust
// At the top of your file, outside any functions
use once_cell::sync::Lazy;
use reqwest::Client;

// Define a static HTTP client that's initialized once
static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
});

// In your handler function
pub async fn get_sensor_status(State(_state): State<AppState>) -> impl IntoResponse {
    // Create a span for sensor status check with request ID for correlation
    let request_id = uuid::Uuid::new_v4().to_string();
    let span = tracing::info_span!(
        "tank_sensor_status_check",
        request_id = %request_id
    );
    let _guard = span.enter();

    // GOOD: Use the static client with idiomatic deref coercion
    let client = &CLIENT;
    // Alternatively: let client = &*CLIENT; // Explicit dereferencing also works
    
    // Log metrics about using the shared client
    tracing::info!(
        request_id = %request_id,
        "Using shared static HTTP client"
    );
    
    // Rest of the function remains the same
    // but now uses the shared static client
}
```

## Solution 3: Combined Approach (Static Client in App State)

This approach combines the benefits of both patterns.

```rust
// At the top of your file, outside any functions
use once_cell::sync::Lazy;
use reqwest::Client;

// Define a static HTTP client that's initialized once
static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
});

// In your AppState struct
pub struct AppState {
    client: &'static Client,
    // ... other state fields as needed
}

// In your main function
let state = AppState { client: &CLIENT };

// In your handler function
pub async fn get_sensor_status(State(state): State<AppState>) -> impl IntoResponse {
    // Use the client from state (which is a reference to the static client)
    let client = state.client;
    
    // Rest of the function remains the same
}
```

## Key Benefits of All Solutions

All of these solutions address the resource leak by ensuring a single HTTP client is shared across all requests. The key optimizations include:

1. Creating the HTTP client once during application startup
2. Reusing the same client for all requests
3. Properly configuring timeouts and connection pools
4. Avoiding the overhead of creating new connections for each request

HTTP clients are resource-intensive objects that maintain connection pools, TLS configurations, and DNS caches. Creating a new one for each request wastes these resources and can cause memory leaks and performance degradation in high-traffic services. By using a shared client, we ensure that these resources are properly managed and reused.

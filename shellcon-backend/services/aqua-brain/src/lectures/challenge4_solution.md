# Challenge 4 Solution: Fixing "The Leaky Connection" in `aqua-monitor`

This document explains how to solve Challenge 4 by optimizing HTTP client usage in the `get_sensor_status` function of the `aqua-monitor` service.

## The Problem: Creating a New HTTP Client Per Request

The original `get_sensor_status` function in `aqua-monitor/src/challenges.rs` created a new `reqwest::Client` every time it was called:

```rust
// In aqua-monitor/src/challenges.rs (Before the fix)
use shuttle_axum::axum::{extract::State, response::IntoResponse};
use crate::AppState; // Assuming AppState is defined
// ... other imports

pub async fn get_sensor_status(State(_state): State<AppState>) -> impl IntoResponse {
    let request_id = uuid::Uuid::new_v4().to_string();
    // ... tracing setup ...

    // ⚠️ PROBLEM: Creating a new client for each request (resource leak)
    let client = reqwest::Client::new();
    
    tracing::info!(
        request_id = %request_id,
        "Created new HTTP client for request"
    );
    
    // ... rest of the function using the new client ...
    // Example:
    // match client.get("https://api.example.com/sensors").send().await { ... }
    serde_json::json!({ "status": "error", "message": "using new client" }) // Placeholder
}
```
This approach leads to resource inefficiency, as explained in the challenge description.

## The Solution: Using Axum's Application State (Recommended for `aqua-monitor`)

The best way to fix this in an Axum application like `aqua-monitor` is to create the `reqwest::Client` once at application startup and share it via Axum's `AppState`.

### Step 1: Modify `AppState` in `aqua-monitor/src/main.rs`

Add the `reqwest::Client` to your `AppState` struct:

```rust
// In aqua-monitor/src/main.rs
use reqwest::Client;
use sqlx::PgPool; // Assuming PgPool is used

#[derive(Clone)]
struct AppState {
    pool: PgPool,        // Existing field
    http_client: Client, // Our new shared HTTP client
}
```

### Step 2: Initialize the Client at Startup in `aqua-monitor/src/main.rs`

In your `axum` main function, create the client and add it to the `AppState`:

```rust
// In aqua-monitor/src/main.rs (axum main function)
use shuttle_axum::axum::{routing::get, Router};
// ... other necessary imports ...

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // ... (database migration if any) ...

    // Create the client once
    let http_client = Client::builder()
        .timeout(std::time::Duration::from_secs(10)) // Example configuration
        .build()
        .expect("Failed to build HTTP client");
    
    // Initialize AppState with the shared client
    let state = AppState { pool, http_client };
    
    let router = Router::new()
        // ... other routes ...
        .route("/api/sensors/status", get(crate::challenges::get_sensor_status)) // Ensure path to handler is correct
        .with_state(state);

    Ok(router.into())
}
```

### Step 3: Use the Shared Client in `aqua-monitor/src/challenges.rs`

Modify `get_sensor_status` to use the client from the `AppState`:

```rust
// In aqua-monitor/src/challenges.rs (After the fix)
use shuttle_axum::axum::{extract::State, response::IntoResponse};
use crate::AppState; // Make sure AppState is accessible
// ... other imports ...

pub async fn get_sensor_status(State(state): State<AppState>) -> impl IntoResponse {
    let request_id = uuid::Uuid::new_v4().to_string();
    // ... tracing setup ...

    // SOLUTION: Use the shared client from AppState
    let client = &state.http_client;
    
    tracing::info!(
        request_id = %request_id,
        "Using shared HTTP client from AppState for request"
    );
    
    // Now, use 'client' to make your HTTP requests
    // Example:
    // match client.get("https://api.example.com/sensors").send().await { ... }
    serde_json::json!({ "status": "ok", "message": "using shared client" }) // Placeholder
}
```

### Why this is better:

*   **Resource Efficiency:** The `reqwest::Client` (with its connection pool and TLS sessions) is created only once.
*   **Performance:** Reusing connections is much faster than establishing new ones.
*   **Clean Architecture:** Follows Axum's recommended pattern for managing shared resources.
*   **Testability:** Easier to mock or provide a test client in `AppState` during testing.

This completes Challenge 4, making the `aqua-monitor` service more robust and performant!
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

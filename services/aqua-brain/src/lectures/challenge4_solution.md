```rust
// Before: Creating a new client for every request
// This causes resource leaks and excessive memory usage
async fn get_sensor_status(State(_state): State<AppState>) -> impl IntoResponse {
    // Create a span for sensor status check
    let span = tracing::info_span!("tank_sensor_status_check");
    let _guard = span.enter();

    // Start timing for performance logging
    let _start = std::time::Instant::now();

    // BAD: Creating a new client for every request
    // This causes memory and resource leaks
    let client = reqwest::Client::new();
    
    // Rest of the function remains the same
    // but creates a new client every time
}

// After optimization: Using a static HTTP client
use once_cell::sync::Lazy;
use reqwest::Client;

// Define a single static HTTP client that is created only once
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
});

async fn get_sensor_status(State(_state): State<AppState>) -> impl IntoResponse {
    // Create a span for sensor status check
    let span = tracing::info_span!("tank_sensor_status_check");
    let _guard = span.enter();

    // Start timing for performance logging
    let _start = std::time::Instant::now();

    // GOOD: Use the shared static client
    // No new client is created here
    let client = &HTTP_CLIENT;
    
    // Rest of the function remains the same
    // but now uses the shared client
}
```

This solution addresses the resource leak by creating a static HTTP client using lazy_static or once_cell instead of creating a new client for every request. The static client is initialized only once and reused across all requests, significantly reducing memory usage and resource consumption. HTTP clients are resource-intensive objects that maintain connection pools, TLS configurations, and DNS caches - creating a new one for each request wastes these resources and can cause memory leaks and performance degradation in high-traffic services.

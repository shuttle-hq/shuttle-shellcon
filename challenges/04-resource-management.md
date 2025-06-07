# Challenge 4: Resource Leak

## The Situation
The tank sensor monitoring system is experiencing degraded performance and occasional failures. The system works initially but becomes increasingly unstable over time.

## Technical Issue
The `get_sensor_status` function in the aqua-monitor service (in `src/challenges.rs`) is creating a new HTTP client for each request, leading to resource leaks and connection pool exhaustion.

## Your Task
Implement proper resource management by using a shared HTTP client instead of creating a new one for each request.

## Hints
1. Look for `reqwest::Client::new()` calls in request handling
2. Consider using `once_cell` for a shared client instance
3. Implement proper connection pooling
4. Check the tracing metrics for client creation events

## Testing Your Solution
After implementing your fix:
1. Verify that only one HTTP client is created
2. Ensure the client is properly shared across requests
3. Check that connection pooling is working
4. Monitor resource usage over time

## Learning Outcomes
This challenge teaches proper resource management in Rust services:
- Implementing singleton patterns with once_cell
- Managing HTTP client lifecycles
- Understanding connection pooling
- Preventing resource leaks in web services

Good luck! Efficient resource management is crucial for stable monitoring systems.

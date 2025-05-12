# Challenge 4: Connection Conundrum

## The Situation
The system connecting to remote tank sensors keeps failing, leaving critical tanks unmonitored. Connections work initially but degrade over time, eventually causing complete monitoring failures.

## Technical Issue
The sensor status API is creating a new HTTP client for every request, causing resource leakage. The problem is in the `get_sensor_status` function in the aqua-monitor service.

## Your Task
Fix the resource management in the `get_sensor_status` function in the aqua-monitor service (in `src/main.rs`).

## Hints
1. Look for instances of creating new clients for every request
2. Consider using a shared, reusable client
3. The `once_cell` crate can be useful for static initialization
4. Check the tracing metrics to see active connection counts

## Testing Your Solution
After implementing your fix:
1. Redeploy the service with `shuttle deploy`
2. Visit the dashboard and check if the Remote Monitoring system status has improved
3. Look at the metrics panel to see if active connection count has stabilized

## Learning Outcomes
This challenge teaches proper resource management in Rust applications:
- Using shared, reusable HTTP clients
- Managing connection pools efficiently
- Using once_cell for lazy static initialization
- Monitoring resource usage with metrics

Good luck! The remote tanks contain some of the most sensitive specimens and need constant monitoring.

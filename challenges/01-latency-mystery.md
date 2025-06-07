# Challenge 1: The Sluggish Sensor

## The Situation
The environmental monitoring system is experiencing severe delays when reading tank configuration files. The system needs to frequently access tank settings, but the current implementation is causing unnecessary blocking and latency.

## Technical Issue
The `get_tank_readings` function in the aqua-monitor service (in `src/challenges.rs`) is using blocking file I/O operations in an async context, causing performance issues when reading tank configuration files.

## Your Task
Optimize the file I/O operations in the `get_tank_readings` function by implementing asynchronous file operations using tokio.

## Hints
1. Look for uses of `std::fs::read_to_string`
2. Consider using `tokio::fs::read_to_string` instead
3. Remember to use `.await` with async operations
4. Check the tracing spans to measure I/O operation duration

## Testing Your Solution
After implementing your fix:
1. The configuration file should be read asynchronously
2. The function should use tokio's async file I/O operations
3. The `io_duration_ms` metric in the logs should show improved performance
4. The overall request latency should be reduced

## Learning Outcomes
This challenge teaches proper async I/O usage in Rust services, specifically:
- Converting blocking I/O to async operations
- Using tokio's file system operations
- Understanding the impact of blocking operations in async contexts
- Measuring I/O performance with metrics

Good luck, engineer! Every millisecond counts when monitoring tank conditions.

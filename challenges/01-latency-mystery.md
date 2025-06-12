# Challenge 1: The Sluggish Sensor

## The Situation
The environmental monitoring system is experiencing severe delays when reading tank configuration files. The system needs to frequently access tank settings, but the current implementation is causing unnecessary blocking and latency.

## Technical Issue
The `get_tank_readings` function in the aqua-monitor service (in `src/challenges.rs`) is using blocking file I/O operations in an async context, causing performance issues when reading tank configuration files.

## Your Task
Optimize the file I/O operations in the `get_tank_readings` function by implementing asynchronous file operations.

## Hints
1. Look for blocking file I/O operations in the code
2. Consider using async alternatives for file operations
3. Remember to use `.await` with async operations
4. Pay attention to how tracing spans are used with async operations - they work differently than with synchronous code!

## Testing Your Solution
After implementing your fix:
1. The configuration file should be read asynchronously
2. The function should use proper async file I/O operations
3. The `io_duration_ms` metric in the logs should show improved performance
4. The overall request latency should be reduced
5. The tracing spans should properly track the async operation duration

## Learning Outcomes
This challenge teaches:
- Converting blocking I/O to async operations
- Using async file system operations
- Understanding the impact of blocking operations in async contexts
- Measuring I/O performance with metrics
- Proper tracing instrumentation in async code

Good luck, engineer! Every millisecond counts when monitoring tank conditions.

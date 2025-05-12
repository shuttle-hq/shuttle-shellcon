# Challenge 1: The Sluggish Sensor

## The Situation
The environmental monitoring system is experiencing severe delays, preventing accurate readings of tank conditions. Tanks are starting to show dangerous temperature and pH levels, but the alerts are coming too late for effective intervention.

## Technical Issue
There appears to be high latency in the sensor data API endpoint. The function `get_tank_readings` in the aqua-monitor service is taking an unusually long time to respond.

## Your Task
Investigate the `get_tank_readings` function in the aqua-monitor service (in `src/main.rs`) and fix the latency issue.

## Hints
1. Look for blocking operations in an async context
2. Check how the database is being queried
3. The tracing metrics show high request duration

## Testing Your Solution
After implementing your fix:
1. Redeploy the service with `shuttle deploy`
2. Visit the dashboard and check if the Environmental Monitoring system status has improved
3. Look at the metrics panel to see if request latency has decreased

## Learning Outcomes
This challenge teaches proper async/await usage in Rust backend services, specifically:
- Avoiding blocking operations in async functions
- Using async database queries correctly
- Understanding how blocking operations affect performance

Good luck, engineer! The crustaceans are counting on you.

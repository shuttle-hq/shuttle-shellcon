# Challenge 5: Analysis Gridlock

## The Situation
The system analyzing multiple data inputs simultaneously bogs down under load, preventing correlation of environmental events. This makes it impossible to detect patterns in tank conditions, hiding potential issues.

## Technical Issue
The analysis API has a concurrency bottleneck due to improper mutex usage. The problem is in the `analyze_tank_conditions` function in the aqua-brain service.

## Your Task
Fix the concurrency issue in the `analyze_tank_conditions` function in the aqua-brain service (in `src/main.rs`).

## Hints
1. Look at how the mutex is being used to protect the cache
2. Consider using a more appropriate synchronization primitive
3. Minimize the duration of lock holding
4. The tokio::sync module provides async-aware synchronization primitives
5. Consider if a Read-Write lock would be more appropriate

## Testing Your Solution
After implementing your fix:
1. Redeploy the service with `shuttle deploy`
2. Visit the dashboard and check if the Analysis Engine system status has improved
3. Look at the metrics panel to see if analysis time has decreased

## Learning Outcomes
This challenge teaches concurrent programming in Rust:
- Using appropriate synchronization primitives
- Minimizing lock contention
- Working with tokio's async-aware synchronization
- Understanding the difference between Mutex and RwLock
- Implementing efficient caching strategies

Good luck! With this final fix, the entire system should be back online!

# Challenge 3: Feeding Frenzy Failure

## The Situation
The automated feeding system crashes when dispensing certain types of food, leading to inconsistent feeding patterns. Some crustaceans haven't been fed in days, while others are being overfed.

## Technical Issue
The feeding schedule API is crashing due to poor error handling in the `get_feeding_schedule` function in the species-hub service.

## Your Task
Implement proper error handling in the `get_feeding_schedule` function in the species-hub service (in `src/main.rs`).

## Hints
1. Look for `panic!` calls that should be handled gracefully
2. Consider implementing proper Rust error handling with Result types
3. Make sure errors are reported to the client with appropriate status codes
4. Use the `?` operator for concise error propagation

## Testing Your Solution
After implementing your fix:
1. Redeploy the service with `shuttle deploy`
2. Visit the dashboard and check if the Feeding System status has improved
3. Verify that the system now handles error cases gracefully rather than crashing

## Learning Outcomes
This challenge teaches proper error handling in Rust web services:
- Using the `?` operator for error propagation
- Creating custom error types with thiserror
- Returning appropriate HTTP status codes for different error cases
- Avoiding panics in production code

Good luck! The feeding system is critical for the well-being of all specimens.

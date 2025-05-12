# Challenge 2: Database Dilemma

## The Situation
The species database is returning incomplete information, making it impossible to determine optimal conditions for each species. This is preventing proper environmental control for rare species, putting them at risk.

## Technical Issue
The species lookup API is performing poorly, with high query times and incomplete results. The problem appears to be in the `get_species` function in the species-hub service.

## Your Task
Optimize the database query in the `get_species` function in the species-hub service (in `src/main.rs`).

## Hints
1. Look at how the LIKE queries are being used
2. Check for missing database indexes
3. Review the SQL query structure for inefficiencies
4. The tracing metrics show high database query times

## Testing Your Solution
After implementing your fix:
1. Add any necessary database indexes in a migration file
2. Redeploy the service with `shuttle deploy`
3. Visit the dashboard and check if the Species Database system status has improved
4. Look at the metrics panel to see if database query time has decreased

## Learning Outcomes
This challenge teaches database optimization techniques in Rust applications:
- Proper use of database indexes
- Optimizing SQL queries with LIKE statements
- Using EXPLAIN to analyze query performance
- Measuring query performance with metrics

Good luck! The rare Marbled Crayfish is particularly sensitive to improper tank conditions and needs your help.

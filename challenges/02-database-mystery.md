# Challenge 2: Database Dilemma

## The Situation
The species search functionality is experiencing performance issues, particularly when searching by name or scientific name. The search is becoming increasingly slow as the species database grows.

## Technical Issue
The `get_species` function in the species-hub service (in `src/challenges.rs`) is using non-indexed LIKE queries, causing full table scans and poor performance.

## Your Task
Optimize the database queries in the `get_species` function by implementing proper indexing and efficient search patterns.

## Hints
1. Look at how case-sensitive LIKE queries are being used
2. Consider using PostgreSQL's trigram indexes for text search
3. Review the query execution plan using EXPLAIN ANALYZE
4. Check the tracing metrics for query duration

## Testing Your Solution
After implementing your fix:
1. Create appropriate indexes for text search
2. Use case-insensitive search where appropriate
3. Verify improved query performance with EXPLAIN ANALYZE
4. Check that the search still works correctly with partial matches

## Learning Outcomes
This challenge teaches database optimization techniques in Rust applications:
- Implementing efficient text search in PostgreSQL
- Using appropriate index types (B-tree, GIN, GiST)
- Understanding query execution plans
- Balancing between search flexibility and performance

Good luck! Our growing collection of aquatic species needs an efficient catalog system.

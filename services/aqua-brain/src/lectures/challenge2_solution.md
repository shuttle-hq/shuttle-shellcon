```rust
// Before: Using LIKE for case-sensitive search
pub async fn get_species(Query(params): Query<SpeciesQuery>, State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // ... setup code omitted for brevity ...
    
    // Using case-sensitive query with LIKE (non-optimized)
    // This requires a full table scan and doesn't utilize indexes effectively
    let species = if let Some(name) = &params.name {
        // Use runtime query with LIKE for case-sensitive search
        sqlx::query("SELECT * FROM species WHERE name LIKE $1")
            .bind(format!("%{}%", name))
            // ... mapping code omitted for brevity ...
            .fetch_all(&state.pool)
            .await?
    } else if let Some(scientific_name) = &params.scientific_name {
        // Use runtime query with LIKE for case-sensitive search
        sqlx::query("SELECT * FROM species WHERE scientific_name LIKE $1")
            .bind(format!("%{}%", scientific_name))
            // ... mapping code omitted for brevity ...
            .fetch_all(&state.pool)
            .await?
    } else {
        // ... default query omitted for brevity ...
    };
    
    // ... rest of function omitted for brevity ...
}

// After: Using ILIKE for case-insensitive search
pub async fn get_species(Query(params): Query<SpeciesQuery>, State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // ... setup code omitted for brevity ...
    
    // Using case-insensitive query with ILIKE (optimized)
    // This can utilize trigram indexes for better performance
    let species = if let Some(name) = &params.name {
        // Use runtime query with ILIKE for case-insensitive search
        sqlx::query("SELECT * FROM species WHERE name ILIKE $1")
            .bind(format!("%{}%", name))
            // ... mapping code omitted for brevity ...
            .fetch_all(&state.pool)
            .await?
    } else if let Some(scientific_name) = &params.scientific_name {
        // Use runtime query with ILIKE for case-insensitive search
        sqlx::query("SELECT * FROM species WHERE scientific_name ILIKE $1")
            .bind(format!("%{}%", scientific_name))
            // ... mapping code omitted for brevity ...
            .fetch_all(&state.pool)
            .await?
    } else {
        // ... default query omitted for brevity ...
    };
    
    // ... rest of function omitted for brevity ...
}
```

```sql
-- Database optimization: Creating trigram indexes for text search
-- These indexes should be added to the database schema

-- Index for the name column
CREATE INDEX IF NOT EXISTS species_name_trigram_idx ON species USING GIN (name gin_trgm_ops);

-- Index for the scientific_name column
CREATE INDEX IF NOT EXISTS species_scientific_name_trigram_idx ON species USING GIN (scientific_name gin_trgm_ops);
```

This solution optimizes database text search performance in two ways:

1. **Using ILIKE instead of LIKE**: The ILIKE operator performs case-insensitive matching, which is usually what users expect when searching. This makes searches more user-friendly and consistent.

2. **Leveraging trigram indexes**: While not shown directly in the code, the solution assumes trigram indexes have been created on the text columns. Trigram indexes break text into three-character segments that can be efficiently searched even with wildcards (% patterns).

These changes can make text searches hundreds of times faster on large tables compared to using unindexed LIKE queries, especially when dealing with wildcard searches that would otherwise require full table scans.

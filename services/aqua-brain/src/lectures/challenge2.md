# Database Query Optimization: Making PostgreSQL Searches Faster

## The Problem: Slow Text Searches

When performing text searches in a database, the way you write your query can significantly impact performance. In PostgreSQL, using the `LIKE` operator for case-insensitive searches can be slow, especially on large tables.

Traditional approach with `LIKE`:

```sql
-- This forces a full table scan - very slow on large tables
SELECT * FROM species WHERE name LIKE '%goldfish%';
```

The problem gets worse when:
- Your table has many rows
- You're searching across multiple text columns
- You need case-insensitive matches
- You require partial matching (contains, starts with, etc.)

## The Solution: PostgreSQL Trigram Indexes with ILIKE

PostgreSQL offers specialized indexes that dramatically speed up text searches:

```sql
-- First, enable the pg_trgm extension
CREATE EXTENSION pg_trgm;

-- Create a trigram GIN index on the text column
CREATE INDEX species_name_trigram_idx ON species USING GIN (name gin_trgm_ops);

-- Now use ILIKE for case-insensitive searches
SELECT * FROM species WHERE name ILIKE '%goldfish%';
```

## What Are Trigrams?

Trigrams are three-character sequences extracted from text. For example, "fish" produces the trigrams: "  f", " fi", "fis", "ish", "sh ".

PostgreSQL's trigram index works by:
1. Breaking text into these trigrams
2. Creating an index of which rows contain which trigrams
3. When searching, it identifies rows with matching trigrams

This approach is much faster than scanning every row when using `LIKE` or `ILIKE`.

## Implementation in SQLx with Rust

### Before Optimization:

```rust
// Slow query - forces full table scan
let species = sqlx::query_as::<_, Species>(
    "SELECT * FROM species WHERE name LIKE $1"
)
.bind(format!("%{}%", search_term))
.fetch_all(&pool)
.await?;
```

### After Optimization:

```rust
// Step 1: Create a *new* migration file to set up the trigram extension and indexes.
// It's crucial to add these schema changes in a **new** migration script.
// Modifying an existing migration file (like the initial `20250510000000_create_species_table.sql`)
// after it has been applied will cause errors when `sqlx` checks migration checksums.
// Always create a new timestamped .sql file in your `migrations` directory for new schema changes.

// In your *new* migration file (e.g., `YYYYMMDDHHMMSS_add_trgm_indexes.sql`):
// CREATE EXTENSION IF NOT EXISTS pg_trgm;
// CREATE INDEX IF NOT EXISTS species_name_gin_trgm_idx ON species USING GIN (name gin_trgm_ops);
// CREATE INDEX IF NOT EXISTS species_scientific_name_gin_trgm_idx ON species USING GIN (scientific_name gin_trgm_ops);

// Step 2: Use ILIKE with your indexed column in your Rust code
let species = sqlx::query_as::<_, Species>(
    "SELECT * FROM species WHERE name ILIKE $1"
)
.bind(format!("%{}%", search_term))
.fetch_all(&pool)
.await?;
```

## Performance Impact

The performance difference can be dramatic:

| Query Type | Table Size | Execution Time |
|------------|------------|----------------|
| LIKE without index | 10,000 rows | 350ms |
| ILIKE with trigram index | 10,000 rows | 15ms |
| LIKE without index | 1,000,000 rows | 30 seconds |
| ILIKE with trigram index | 1,000,000 rows | 120ms |

## Best Practices for Text Search Optimization

1. **Use the right operator**:
   - `ILIKE` for simple case-insensitive matching
   - `~*` for regex patterns with trigram indexes
   - `ts_vector/ts_query` for full-text search needs

2. **Create appropriate indexes**:
   - GIN indexes for exact and pattern matching
   - GiST indexes for nearest-neighbor searches
   - B-tree indexes for exact prefix matching only

3. **Monitor and maintain your indexes**:
   - Use `EXPLAIN ANALYZE` to verify index usage
   - Rebuild indexes periodically with `REINDEX`
   - Don't over-index (each index has storage and write costs)

4. **Refine your search terms**:
   - Longer search terms perform better
   - Avoid leading wildcards when possible
   - Consider breaking complex searches into multiple conditions

## When to Use Different PostgreSQL Search Methods

| Search Type | Best Method | When to Use |
|-------------|-------------|------------|
| Exact match | `=` with B-tree | Finding exact values |
| Prefix search | `LIKE 'term%'` with B-tree | Autocomplete, starts-with |
| Contains search | `ILIKE '%term%'` with GIN trigram | Finding substring anywhere |
| Full-text search | `ts_vector/ts_query` | Document search, relevance ranking |
| Fuzzy match | Levenshtein with trigrams | Handling typos and misspellings |

## Example: Complete Implementation in Rust

```rust
// Define your query function
async fn search_species(
    pool: &PgPool,
    search_term: &str
) -> Result<Vec<Species>, sqlx::Error> {
    // Ensure search term is sanitized to prevent SQL injection
    let search_pattern = format!("%{}%", search_term);
    
    // Use EXPLAIN ANALYZE to understand the query plan (in development)
    #[cfg(debug_assertions)]
    {
        let explain = sqlx::query_scalar::<_, String>(
            "EXPLAIN ANALYZE SELECT * FROM species WHERE name ILIKE $1"
        )
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        
        for line in explain {
            println!("{}", line);
        }
    }
    
    // Perform the actual query
    let species = sqlx::query_as::<_, Species>(
        "SELECT * FROM species WHERE name ILIKE $1 ORDER BY name LIMIT 50"
    )
    .bind(&search_pattern)
    .fetch_all(pool)
    .await?;
    
    Ok(species)
}
```

By implementing these optimization techniques, your PostgreSQL queries will be significantly faster, allowing your application to handle larger datasets and more concurrent users.

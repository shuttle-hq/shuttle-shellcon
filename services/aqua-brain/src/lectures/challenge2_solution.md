```rust
// In services/species-hub/src/challenges.rs
// The final get_species function using sqlx::QueryBuilder.
pub async fn get_species(
    Query(params): Query<SpeciesQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let start = std::time::Instant::now();
    let span = tracing::info_span!("species_catalog_search", %request_id);
    let _guard = span.enter();

    tracing::info!(
        request_id = %request_id,
        operation = "species_catalog_search",
        search_by_name = params.name.is_some(),
        search_by_scientific_name = params.scientific_name.is_some(),
        "Starting species catalog search"
    );

    if let Some(name) = &params.name {
        if name.len() < 2 {
            return Err(ApiError::InvalidQuery("Name search term must be at least 2 characters".to_string()));
        }
    }
    if let Some(scientific_name) = &params.scientific_name {
        if scientific_name.len() < 2 {
            return Err(ApiError::InvalidQuery("Scientific name search term must be at least 2 characters".to_string()));
        }
    }

    let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM species WHERE 1=1");
    let mut has_conditions = false;

    if let Some(name) = &params.name {
        query_builder.push(" AND name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        has_conditions = true;
    }

    if let Some(scientific_name) = &params.scientific_name {
        query_builder.push(" AND scientific_name ILIKE ");
        query_builder.push_bind(format!("%{}%", scientific_name));
        has_conditions = true;
    }

    if !has_conditions {
        query_builder.push(" LIMIT 20");
    } else {
        query_builder.push(" ORDER BY name LIMIT 50");
    }

    let query = query_builder.build_query_as::<Species>();

    let species_result = query
        .fetch_all(&state.pool)
        .await;

    let species = match species_result {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(
                request_id = %request_id,
                error.type = "database",
                error.message = %e,
                "Error executing species search query"
            );
            return Err(ApiError::Database(e));
        }
    };

    let elapsed = start.elapsed().as_millis();
    tracing::info!(
        request_id = %request_id,
        operation = "species_catalog_search",
        operation_status = "success",
        query_duration_ms = elapsed as f64,
        results_count = species.len(),
        db_query_type = if has_conditions { "species_search_optimized" } else { "species_list_all" },
        search_term = params.name.as_deref().unwrap_or_else(|| params.scientific_name.as_deref().unwrap_or("N/A")),
        "Species catalog search completed"
    );

    Ok(Json(species))
}
```

This challenge requires two main steps: optimizing the database schema by creating a new migration, and updating the query logic in your Rust code.

**Step 1: Optimize the Database Schema with a New Migration**

Database migrations are like version control for your database schema. They allow you to evolve your database structure in a consistent and trackable way. In Rust projects using `sqlx` (which Shuttle utilizes for database provisioning), migrations are typically SQL files located in a `migrations` directory within your service. These files are named with a timestamp prefix (e.g., `YYYYMMDDHHMMSS_description.sql`) to ensure they are applied in chronological order.

When your application starts, `sqlx` checks this `migrations` directory, compares it against a special history table it maintains in your database (often named `_sqlx_migrations`), and applies any new, unapplied migration scripts. This history table records which migrations have been run and a checksum of their content.

**Crucially, you should never modify a migration file after it has been successfully applied to any database environment (even local).** If `sqlx` detects that an already-applied migration script has been altered (its checksum changes), it will raise an error and refuse to proceed to prevent potential schema corruption or inconsistencies. This is a critical safety feature.

Instead, for any new schema changes, the correct practice is to create a **new migration file**.

For this challenge, you need to:
1.  **Create a new SQL migration file** in your `services/species-hub/migrations/` directory. A good name would follow the timestamp convention, for example: `20250615120000_add_trgm_and_indexes_for_species.sql`. (Remember to replace `20250615120000` with the actual current timestamp when you create the file to ensure it runs after any existing migrations).
2.  Add the following SQL content to this **new file**:

    ```sql
    -- Enable the pg_trgm extension for trigram-based text searching.
    -- This extension provides functions and operators for determining the similarity
    -- of alphanumeric text based on trigram matching, which is essential for
    -- efficient partial string searches (LIKE/ILIKE with wildcards).
    CREATE EXTENSION IF NOT EXISTS pg_trgm;

    -- Create GIN indexes using trigram operations for efficient ILIKE searches.
    -- GIN (Generalized Inverted Index) is well-suited for indexing composite values
    -- or types where elements within them need to be searched (like text documents or, in this case, trigrams from text).
    -- 'gin_trgm_ops' specifies that the GIN index should use trigram operations provided by pg_trgm.
    -- These indexes will significantly speed up case-insensitive searches on 'name' and 'scientific_name'.
    CREATE INDEX IF NOT EXISTS species_name_gin_trgm_idx ON species USING GIN (name gin_trgm_ops);
    CREATE INDEX IF NOT EXISTS species_scientific_name_gin_trgm_idx ON species USING GIN (scientific_name gin_trgm_ops);
    ```

When your `species-hub` service next starts, `sqlx::migrate!` will automatically detect this new migration file and apply it to your database, adding the extension and indexes.

**Step 2: Update Rust Code to Use `ILIKE` with `sqlx::QueryBuilder`**

After preparing the database schema with the `pg_trgm` extension and GIN indexes via your new migration, you need to modify your Rust code in `services/species-hub/src/challenges.rs`. The `get_species` function should be updated to use `sqlx::QueryBuilder` for dynamically constructing the SQL query. This approach allows for more flexible query building, especially when dealing with optional search parameters.

Within the `get_species` function, you will use `query_builder.push()` to add segments to your SQL query and `query_builder.push_bind()` to safely bind parameter values. The key is to use `ILIKE` for case-insensitive pattern matching, which can leverage the trigram GIN indexes you've created.

For example, to add a condition for searching the `name` column:
```rust
if let Some(name) = &params.name {
    query_builder.push(" AND name ILIKE ");
    query_builder.push_bind(format!("%{}%", name));
    has_conditions = true;
}
```
This dynamically adds `AND name ILIKE $N` (where `$N` is a placeholder for the bound parameter) to your query if a name parameter is provided.

`ILIKE` is PostgreSQL's case-insensitive version of `LIKE`. It allows for pattern matching regardless of letter casing. When combined with trigram GIN indexes, this results in fast, case-insensitive searches.

The complete updated Rust code for `get_species` using `sqlx::QueryBuilder` is shown in the snippet at the beginning of this solution.

**Why these changes are effective:**

*   **`pg_trgm` and GIN Indexes:** Standard `LIKE` queries, especially with leading wildcards (e.g., `'%searchterm%'`), on unindexed text fields are notoriously slow on large datasets. They often force the database to perform a full table scan, examining every row. The `pg_trgm` extension allows PostgreSQL to break down text into trigrams (sequences of three characters). GIN indexes built on these trigrams enable the database to quickly find matching trigrams and, by extension, matching strings, dramatically speeding up searches.
*   **`ILIKE` Operator:** This operator provides the case-insensitivity often desired in user-facing search functionalities. When used with trigram indexes, it offers a powerful and performant search solution.
*   **Adherence to Migration Best Practices:** Creating new migration files for schema changes is fundamental to robust database management. It ensures that schema evolution is orderly, repeatable, and traceable across all environments (development, testing, production).

By implementing these two steps—creating a new migration for schema enhancements and updating your Rust code to use `ILIKE`—your species search functionality will become significantly faster and more aligned with user expectations for search behavior.

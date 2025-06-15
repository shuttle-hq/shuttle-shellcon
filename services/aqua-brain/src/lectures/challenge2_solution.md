```rust
// In services/species-hub/src/challenges.rs
// The final get_species function after applying ILIKE and ensuring correct query construction.

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json, Extension};
use serde::Deserialize;
use sqlx::{Postgres, Row, postgres::PgRow};
use uuid::Uuid;
use crate::{AppState, ApiError, Species}; // Assuming Species and ApiError are in scope

#[derive(Deserialize)]
pub struct SpeciesQuery {
    pub name: Option<String>,
    pub scientific_name: Option<String>,
}

pub async fn get_species(
    Query(params): Query<SpeciesQuery>,
    State(state): State<AppState>,
    Extension(request_id): Extension<Uuid> // Assuming request_id is passed as an extension
) -> Result<impl IntoResponse, ApiError> {
    let start = std::time::Instant::now();
    let mut query_conditions = Vec::new();
    let mut query_params_values: Vec<String> = Vec::new(); // To hold the actual string values for binding

    let mut param_idx = 1;

    if let Some(name) = &params.name {
        query_conditions.push(format!("name ILIKE ${}", param_idx));
        query_params_values.push(format!("%{}%", name));
        param_idx += 1;
    }
    if let Some(scientific_name) = &params.scientific_name {
        query_conditions.push(format!("scientific_name ILIKE ${}", param_idx));
        query_params_values.push(format!("%{}%", scientific_name));
        // param_idx += 1; // No need to increment if it's the last one
    }

    let species_list: Vec<Species> = if !query_conditions.is_empty() {
        let query_str = format!(
            "SELECT id, name, scientific_name, description, min_temperature, max_temperature, min_ph, max_ph, diet_type FROM species WHERE {} LIMIT 20",
            query_conditions.join(" AND ")
        );
        
        let mut running_query = sqlx::query_as::<Postgres, Species>(&query_str);
        for val in query_params_values {
            running_query = running_query.bind(val);
        }

        running_query
            .fetch_all(&state.pool)
            .await
            .map_err(|e| {
                tracing::error!(request_id = %request_id, error.type = "database", error.message = %e, "Error executing species list query with conditions");
                ApiError::Database(e)
            })?
    } else {
        sqlx::query_as::<Postgres, Species>("SELECT id, name, scientific_name, description, min_temperature, max_temperature, min_ph, max_ph, diet_type FROM species LIMIT 20")
            .fetch_all(&state.pool)
            .await
            .map_err(|e| {
                tracing::error!(request_id = %request_id, error.type = "database", error.message = %e, "Error executing get all species query");
                ApiError::Database(e)
            })?
    };
    
    let elapsed = start.elapsed().as_millis();
    tracing::info!(request_id = %request_id, "Species query executed in {}ms, found {} results", elapsed, species_list.len());

    Ok((StatusCode::OK, Json(species_list)))
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

**Step 2: Update Rust Code to Use `ILIKE`**

After preparing the database schema with the `pg_trgm` extension and GIN indexes via your new migration, you need to modify your Rust code in `services/species-hub/src/challenges.rs`. Specifically, within the `get_species` function, change the SQL queries to use the `ILIKE` operator instead of `LIKE`.

`ILIKE` is PostgreSQL's case-insensitive version of `LIKE`. It allows for pattern matching regardless of letter casing and, importantly, can leverage the trigram GIN indexes you've just created. This combination results in fast, case-insensitive searches.

The updated Rust code for `get_species` is shown in the snippet at the beginning of this solution. Key changes involve replacing `LIKE` with `ILIKE` in your query strings.

**Why these changes are effective:**

*   **`pg_trgm` and GIN Indexes:** Standard `LIKE` queries, especially with leading wildcards (e.g., `'%searchterm%'`), on unindexed text fields are notoriously slow on large datasets. They often force the database to perform a full table scan, examining every row. The `pg_trgm` extension allows PostgreSQL to break down text into trigrams (sequences of three characters). GIN indexes built on these trigrams enable the database to quickly find matching trigrams and, by extension, matching strings, dramatically speeding up searches.
*   **`ILIKE` Operator:** This operator provides the case-insensitivity often desired in user-facing search functionalities. When used with trigram indexes, it offers a powerful and performant search solution.
*   **Adherence to Migration Best Practices:** Creating new migration files for schema changes is fundamental to robust database management. It ensures that schema evolution is orderly, repeatable, and traceable across all environments (development, testing, production).

By implementing these two steps—creating a new migration for schema enhancements and updating your Rust code to use `ILIKE`—your species search functionality will become significantly faster and more aligned with user expectations for search behavior.

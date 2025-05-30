```sql
-- Before: Using LIKE for case-insensitive search
-- This forces a full table scan - no index use
SELECT * FROM species WHERE name LIKE '%seahorse%'

-- After: Using ILIKE with trigram index
-- First ensure index exists:
CREATE INDEX IF NOT EXISTS species_name_trigram_idx ON species USING GIN (name gin_trgm_ops);

-- Then use ILIKE:
SELECT * FROM species WHERE name ILIKE '%seahorse%'
```

This solution optimizes database text search performance by creating a trigram index (GIN) and using PostgreSQL's ILIKE operator instead of LIKE. Trigram indexes break text into three-character segments that can be efficiently searched even with wildcards. ILIKE is case-insensitive by default and works well with these indexes. This approach can make text searches hundreds of times faster on large tables compared to using unindexed LIKE queries.

-- Enable the pg_trgm extension for trigram-based text searching
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Create GIN indexes using trigram operations for efficient ILIKE searches
-- These indexes will speed up queries using ILIKE on the name and scientific_name columns.
CREATE INDEX IF NOT EXISTS species_name_gin_trgm_idx ON species USING GIN (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS species_scientific_name_gin_trgm_idx ON species USING GIN (scientific_name gin_trgm_ops);

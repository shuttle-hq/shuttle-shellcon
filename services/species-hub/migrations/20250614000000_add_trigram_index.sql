-- Add PostgreSQL trigram extension for efficient text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Add trigram indexes for text search optimization on species name and scientific_name
CREATE INDEX IF NOT EXISTS species_name_trigram_idx ON species USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS species_scientific_name_trigram_idx ON species USING gin (scientific_name gin_trgm_ops);

-- This migration adds trigram indexes to support efficient ILIKE queries with wildcards
-- These indexes will significantly improve performance for queries like:
-- SELECT * FROM species WHERE name ILIKE '%search_term%'

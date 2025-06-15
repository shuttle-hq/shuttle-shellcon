-- Create species table
CREATE TABLE IF NOT EXISTS species (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    scientific_name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    min_temperature FLOAT NOT NULL,
    max_temperature FLOAT NOT NULL,
    min_ph FLOAT NOT NULL,
    max_ph FLOAT NOT NULL,
    diet_type VARCHAR(50) NOT NULL
);

-- Enable the pg_trgm extension for trigram-based text searching (similarity matching)
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Create GIN indexes using trigram operations for efficient ILIKE searches
-- These indexes will significantly speed up partial and case-insensitive text searches
-- on the name and scientific_name columns as required by Challenge 2.
CREATE INDEX IF NOT EXISTS species_name_gin_trgm_idx ON species USING GIN (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS species_scientific_name_gin_trgm_idx ON species USING GIN (scientific_name gin_trgm_ops);

-- Insert sample data
INSERT INTO species (name, scientific_name, description, min_temperature, max_temperature, min_ph, max_ph, diet_type)
VALUES 
    ('Mantis Shrimp', 'Odontodactylus scyllarus', 'The peacock mantis shrimp is known for its powerful strikes and vibrant coloration.', 22.0, 28.0, 8.0, 8.4, 'carnivore'),
    ('Blue Lobster', 'Homarus gammarus', 'The European lobster with a striking blue coloration.', 15.0, 18.0, 7.8, 8.2, 'carnivore'),
    ('Vampire Crab', 'Geosesarma dennerle', 'Small freshwater crabs with vibrant colors, often kept in paludariums.', 24.0, 29.0, 7.0, 7.5, 'omnivore'),
    ('Amano Shrimp', 'Caridina multidentata', 'Algae-eating freshwater shrimp popular in planted aquariums.', 20.0, 26.0, 6.5, 7.5, 'herbivore'),
    ('Red Cherry Shrimp', 'Neocaridina davidi', 'Small bright red freshwater shrimp, excellent for community tanks.', 20.0, 28.0, 6.5, 8.0, 'herbivore'),
    ('Mexican Dwarf Crayfish', 'Cambarellus patzcuarensis', 'Miniature crayfish suitable for smaller aquariums.', 20.0, 25.0, 7.0, 8.0, 'omnivore'),
    ('Thai Micro Crab', 'Limnopilos naiyanetri', 'Tiny freshwater crabs often found in plant roots.', 22.0, 28.0, 6.5, 7.5, 'omnivore'),
    ('Ghost Shrimp', 'Palaemonetes paludosus', 'Transparent freshwater shrimp used as tank cleaners.', 20.0, 26.0, 7.0, 8.0, 'omnivore'),
    ('Bamboo Shrimp', 'Atyopsis moluccensis', 'Filter-feeding freshwater shrimp with fan-like appendages.', 23.0, 28.0, 6.5, 7.5, 'filter feeder'),
    ('Pom Pom Crab', 'Lybia tessellata', 'Crabs that hold sea anemones in their claws for defense.', 22.0, 26.0, 8.1, 8.4, 'omnivore'),
    ('Marble Crayfish', 'Procambarus virginalis', 'Self-cloning crayfish with marble-like patterns.', 18.0, 25.0, 7.0, 8.0, 'omnivore'),
    ('Tiger Pistol Shrimp', 'Alpheus bellulus', 'Shrimp that can create a snapping sound with its claw.', 23.0, 28.0, 8.1, 8.4, 'carnivore');

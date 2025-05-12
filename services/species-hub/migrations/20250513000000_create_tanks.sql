-- Create tanks reference table in species-hub
-- This table mirrors data from aqua-monitor but is maintained independently
CREATE TABLE IF NOT EXISTS tanks (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    tank_type VARCHAR(50) NOT NULL,
    volume FLOAT NOT NULL,
    description TEXT
);

-- Insert sample data with exact tank IDs that match existing tank readings
-- These tank types will be used by the feeding schedule API
INSERT INTO tanks (id, name, tank_type, volume, description)
VALUES 
    ('Tank-A1', 'Reef Display', 'reef', 200.0, 'Large reef display tank with corals and reef-safe fish'),
    ('Tank-B2', 'Community Tank', 'community', 150.0, 'Mixed species community tank'),
    ('Tank-C3', 'Nano Shrimp Tank', 'nano', 20.0, 'Small tank specifically for shrimp');

-- Note: In a real-world application, this table would be synchronized 
-- with the tanks table in aqua-monitor through a data synchronization process

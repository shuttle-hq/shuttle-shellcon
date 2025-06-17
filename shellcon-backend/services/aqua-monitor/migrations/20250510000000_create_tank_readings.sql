-- Create tank_readings table
CREATE TABLE IF NOT EXISTS tank_readings (
    id SERIAL PRIMARY KEY,
    tank_id VARCHAR(50) NOT NULL,
    temperature FLOAT NOT NULL,
    ph FLOAT NOT NULL,
    oxygen_level FLOAT NOT NULL,
    salinity FLOAT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on tank_id for faster queries
CREATE INDEX IF NOT EXISTS idx_tank_readings_tank_id ON tank_readings(tank_id);

-- Insert sample data
INSERT INTO tank_readings (tank_id, temperature, ph, oxygen_level, salinity, timestamp)
VALUES 
    ('Tank-A1', 25.2, 7.8, 8.5, 35.1, NOW() - INTERVAL '1 hour'),
    ('Tank-A1', 25.3, 7.7, 8.3, 35.0, NOW() - INTERVAL '45 minutes'),
    ('Tank-A1', 25.5, 7.6, 8.2, 34.9, NOW() - INTERVAL '30 minutes'),
    ('Tank-A1', 25.6, 7.5, 8.0, 34.8, NOW() - INTERVAL '15 minutes'),
    ('Tank-A1', 25.8, 7.4, 7.8, 34.7, NOW()),
    ('Tank-B2', 22.5, 8.1, 9.0, 30.5, NOW() - INTERVAL '1 hour'),
    ('Tank-B2', 22.6, 8.0, 8.9, 30.4, NOW() - INTERVAL '45 minutes'),
    ('Tank-B2', 22.7, 7.9, 8.8, 30.3, NOW() - INTERVAL '30 minutes'),
    ('Tank-B2', 22.8, 7.8, 8.7, 30.2, NOW() - INTERVAL '15 minutes'),
    ('Tank-B2', 22.9, 7.7, 8.6, 30.1, NOW()),
    ('Tank-C3', 18.1, 6.9, 7.5, 25.5, NOW() - INTERVAL '1 hour'),
    ('Tank-C3', 18.2, 6.8, 7.4, 25.4, NOW() - INTERVAL '45 minutes'),
    ('Tank-C3', 18.3, 6.7, 7.3, 25.3, NOW() - INTERVAL '30 minutes'),
    ('Tank-C3', 18.4, 6.6, 7.2, 25.2, NOW() - INTERVAL '15 minutes'),
    ('Tank-C3', 18.5, 6.5, 7.1, 25.1, NOW());

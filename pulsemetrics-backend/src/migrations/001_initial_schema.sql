-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create events table
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    time TIMESTAMPTZ NOT NULL,
    project_id VARCHAR(100) NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    properties JSONB,
    user_id VARCHAR(100),
    session_id UUID,
    value DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Convert to hypertable (TimescaleDB)
SELECT create_hypertable('events', 'time', if_not_exists => TRUE);

-- Create indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_events_project_time 
    ON events (project_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_events_project_type_time 
    ON events (project_id, event_type, time DESC);

CREATE INDEX IF NOT EXISTS idx_events_user_time 
    ON events (user_id, time DESC) 
    WHERE user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_events_session_time 
    ON events (session_id, time DESC) 
    WHERE session_id IS NOT NULL;

-- GIN index for JSONB properties
CREATE INDEX IF NOT EXISTS idx_events_properties 
    ON events USING GIN (properties);

-- Set retention policy (optional: keep data for 90 days)
-- SELECT add_retention_policy('events', INTERVAL '90 days', if_not_exists => TRUE);

-- Create a view for event counts (useful for monitoring)
CREATE OR REPLACE VIEW event_stats AS
SELECT 
    project_id,
    event_type,
    COUNT(*) as count,
    MIN(time) as first_seen,
    MAX(time) as last_seen
FROM events
GROUP BY project_id, event_type;

-- Comment the table
COMMENT ON TABLE events IS 'Time-series events table managed by TimescaleDB';
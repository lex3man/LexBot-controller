CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    value VARCHAR(255) NOT NULL UNIQUE,
    user_id VARCHAR(100) NOT NULL,
    active BOOLEAN DEFAULT FALSE,
    life_time_minutes INT CHECK (life_time_minutes >= 0) NOT NULL,
    created_at TIMESTAMP
        WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP
        WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS tokens_value_idx ON tokens (value);
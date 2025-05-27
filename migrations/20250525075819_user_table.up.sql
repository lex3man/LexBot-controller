CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username VARCHAR(255) NOT NULL UNIQUE,
    pass_hash VARCHAR(100) NOT NULL,
    active BOOLEAN DEFAULT FALSE,
    user_group VARCHAR(100) NOT NULL,
    created_at TIMESTAMP
        WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP
        WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);
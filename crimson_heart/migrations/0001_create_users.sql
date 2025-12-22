CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    username STRING NOT NULL,
    password STRING NOT NULL,
    email STRING UNIQUE NOT NULL,
    birth_date DATE NOT NULL,
    created_at DATE NOT NULL
);
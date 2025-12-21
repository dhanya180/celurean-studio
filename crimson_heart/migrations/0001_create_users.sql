CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    username STRING NOT NULL,
    email STRING UNIQUE NOT NULL,
    birth_date DATE
);
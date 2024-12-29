-- Your SQL goes here
CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('threads');
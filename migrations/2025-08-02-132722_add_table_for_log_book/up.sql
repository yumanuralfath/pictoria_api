-- Your SQL goes here
CREATE TABLE log_books (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    content TEXT NOT NULL
);
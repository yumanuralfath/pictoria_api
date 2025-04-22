-- Your SQL goes here
CREATE TABLE voices_months (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  voice_id INTEGER NOT NULL REFERENCES voices_weeks(id) ON DELETE CASCADE,
  voices_month_journal TEXT NOT NULL,
  month VARCHAR(7) NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE (user_id, month)
);

-- Tabel voices_weeks
CREATE TABLE voices_weeks (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  voice_id INTEGER NOT NULL REFERENCES voices(id) ON DELETE CASCADE,
  voices_week_journal TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
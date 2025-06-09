-- Your SQL goes here
CREATE TABLE voices_months_voices (
  id SERIAL PRIMARY KEY,
  voices_month_id INTEGER NOT NULL REFERENCES voices_months(id) ON DELETE CASCADE,
  voice_id INTEGER NOT NULL REFERENCES voices(id) ON DELETE CASCADE,
  UNIQUE (voices_month_id, voice_id)
);

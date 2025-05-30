-- Your SQL goes here
CREATE TABLE voices_weeks_voices (
  id SERIAL PRIMARY KEY,
  voices_week_id INTEGER NOT NULL REFERENCES voices_weeks(id) ON DELETE CASCADE,
  voice_id INTEGER NOT NULL REFERENCES voices(id) ON DELETE CASCADE,
  UNIQUE (voices_week_id, voice_id)
);

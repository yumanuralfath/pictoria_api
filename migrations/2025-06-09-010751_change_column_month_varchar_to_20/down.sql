-- This file should undo anything in `up.sql`
ALTER TABLE voices_months ALTER COLUMN month TYPE VARCHAR(7);
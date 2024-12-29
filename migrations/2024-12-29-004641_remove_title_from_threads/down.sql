-- This file should undo anything in `up.sql`
ALTER TABLE threads ADD COLUMN title VARCHAR NOT NULL DEFAULT '';

-- This file should undo anything in `up.sql`

ALTER TABLE wallets DROP COLUMN profile_id;

DROP TABLE profiles;

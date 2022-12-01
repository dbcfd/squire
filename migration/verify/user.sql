-- Verify squire:user on pg

BEGIN;

SELECT id, email, hashed_pasword, salt, created_at, updated_at, deleted_at
  FROM squire.user
 WHERE FALSE;

ROLLBACK;

-- Verify squire:user on pg

BEGIN;

SELECT id, email, hashed_pasword, created_at, updated_at, deleted_at
  FROM squire.user
 WHERE FALSE;

ROLLBACK;

-- Verify squire:city on pg

BEGIN;

SELECT id, user, city, country, created_at, updated_at, deleted_at
  FROM squire.user
 WHERE FALSE;


ROLLBACK;

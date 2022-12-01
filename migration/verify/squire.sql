-- Verify squire:squire on pg

BEGIN;

SELECT pg_catalog.has_schema_privilege('squire', 'usage');

ROLLBACK;

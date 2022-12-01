-- Revert squire:city from pg

BEGIN;

DROP TABLE "squire"."city";

COMMIT;

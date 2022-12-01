-- Revert squire:user from pg

BEGIN;

DROP TABLE "squire"."user";

COMMIT;

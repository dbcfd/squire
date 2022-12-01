-- Deploy squire:user to pg
-- requires: squire

BEGIN;

CREATE TABLE "squire"."user" (
  "id" uuid PRIMARY KEY default gen_random_uuid(),
  "email" varchar NOT NULL,
  "hashed_pasword" varchar NOT NULL,
  "salt" varchar NOT NULL,
  "created_at" timestamptz NOT NULL default now(),
  "updated_at" timestamptz NOT NULL default now(),
  "deleted_at" timestamptz,
);

CREATE INDEX ON "squire"."user" ("email");

COMMIT;

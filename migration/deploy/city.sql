-- Deploy squire:city to pg
-- requires: user

BEGIN;

CREATE TABLE "squire"."city" (
  "id" uuid PRIMARY KEY NOT NULL default gen_random_uuid(),
  "user" uuid NOT NULL,
  "city" varchar NOT NULL,
  "country" varchar NOT NULL,
  "created_at" timestamptz NOT NULL default now(),
  "updated_at" timestamptz NOT NULL default now(),
  "deleted_at" timestamptz
);

ALTER TABLE "squire"."city" ADD FOREIGN KEY ("user") REFERENCES "squire"."user" ("id");

CREATE INDEX ON "squire"."city" ("user", "country");

COMMIT;

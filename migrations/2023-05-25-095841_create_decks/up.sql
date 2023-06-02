CREATE TABLE "decks"
(
    "id"      SERIAL PRIMARY KEY,
    "user_id" INTEGER   NOT NULL,
    "from"    TEXT      NOT NULL,
    "to"      TEXT      NOT NULL,
    "seen_at" TIMESTAMP NOT NULL
);
CREATE TABLE "decks"
(
    "id"     SERIAL PRIMARY KEY,
    "userId" INTEGER   NOT NULL,
    "from"   TEXT      NOT NULL,
    "to"     TEXT      NOT NULL,
    "seenAt" TIMESTAMP NOT NULL
);
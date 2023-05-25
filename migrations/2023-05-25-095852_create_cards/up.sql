CREATE TABLE "cards"
(
    "id"         SERIAL PRIMARY KEY,
    "deckId"     INTEGER   NOT NULL,
    "from"       TEXT      NOT NULL,
    "to"         TEXT      NOT NULL,
    "example"    TEXT,
    "audioUrl"   TEXT,
    "seenAt"     TIMESTAMP NOT NULL,
    "seenFor"    INTEGER   NOT NULL,
    "rating"     INTEGER   NOT NULL,
    "prevRating" INTEGER   NOT NULL,
    "related"    INTEGER[] NOT NULL
);
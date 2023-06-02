CREATE TABLE "cards"
(
    "id"          SERIAL PRIMARY KEY,
    "user_id"     INTEGER   NOT NULL,
    "deck_id"     INTEGER   NOT NULL,
    "from"        TEXT      NOT NULL,
    "to"          TEXT      NOT NULL,
    "example"     TEXT,
    "audio_url"   TEXT,
    "seen_at"     TIMESTAMP NOT NULL,
    "seen_for"    INTEGER   NOT NULL,
    "rating"      INTEGER   NOT NULL,
    "prev_rating" INTEGER   NOT NULL,
    "related"     INTEGER[] NOT NULL
);
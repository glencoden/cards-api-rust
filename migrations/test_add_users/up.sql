CREATE TABLE "users"
(
    "id"    SERIAL PRIMARY KEY,
    "name"  TEXT NOT NULL,
    "first" TEXT NOT NULL,
    "last"  TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "pw"    TEXT NOT NULL
);
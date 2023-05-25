CREATE TABLE "users"
(
    "id"    serial PRIMARY KEY,
    "name"  text NOT NULL,
    "first" text NOT NULL,
    "last"  text NOT NULL,
    "email" text NOT NULL
);
CREATE TABLE IF NOT EXISTS "quotes"
(
    "id"              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "quote_body"      TEXT,
    "user_id"         INTEGER NOT NULL,
    "created_at"      TIMESTAMP,
    "poster_id"       INTEGER NOT NULL,
    "rating"          INTEGER NOT NULL,
    "parent_quote_id" INTEGER
);


CREATE TABLE IF NOT EXISTS "users"
(
    "id"                  INTEGER     NOT NULL PRIMARY KEY AUTOINCREMENT,
    "username"            VARCHAR(50) NOT NULL,
    "firstname"           VARCHAR(50) NOT NULL,
    "surname"             VARCHAR(50) NOT NULL,
    "password_hash"       VARCHAR(50) NOT NULL,
    "email"               VARCHAR(50) DEFAULT 'user@example.com',
    "last_posted"         INTEGER,
    "favourite_quote_id"  INTEGER,
    "reset_token"         VARCHAR(50),
    "reset_token_expires" INTEGER
);


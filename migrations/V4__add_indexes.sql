-- users
CREATE UNIQUE INDEX idx_users_username ON users (username);
CREATE UNIQUE INDEX idx_users_email ON users (email);
CREATE UNIQUE INDEX idx_users_reset_token ON users (reset_token);

-- quotes
CREATE INDEX idx_quotes_user_id ON quotes (user_id);
CREATE INDEX idx_quotes_poster_id ON quotes (poster_id);
CREATE INDEX idx_quotes_poster_id_and_user_id ON quotes (poster_id, user_id);

-- ratings
CREATE INDEX idx_ratings_quote_id ON ratings (quote_id);
CREATE UNIQUE INDEX idx_ratings_quote_id_and_user_id ON ratings (quote_id, user_id);

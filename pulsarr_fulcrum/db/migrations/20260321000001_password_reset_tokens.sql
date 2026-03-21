CREATE TABLE password_reset_token (
    token_hash TEXT NOT NULL PRIMARY KEY,
    pulsarr_user_id INTEGER NOT NULL
        REFERENCES pulsarr_user (pulsarr_user_id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX idx_password_reset_token_user ON password_reset_token (pulsarr_user_id);
CREATE INDEX idx_password_reset_token_expires ON password_reset_token (expires_at);

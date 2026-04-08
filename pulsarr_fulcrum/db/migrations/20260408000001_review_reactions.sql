-- Emoji reactions on reviews (ratings)
CREATE TABLE rating_reaction
(
    rating_reaction_id serial
        CONSTRAINT rating_reaction_pk
            PRIMARY KEY,
    rating_id integer NOT NULL
        CONSTRAINT rating_reaction_rating_fk
            REFERENCES rating (rating_id) ON DELETE CASCADE,
    pulsarr_user_id integer NOT NULL
        CONSTRAINT rating_reaction_user_fk
            REFERENCES pulsarr_user (pulsarr_user_id) ON DELETE CASCADE,
    emoji text NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT rating_reaction_unique UNIQUE (rating_id, pulsarr_user_id, emoji)
);

CREATE INDEX rating_reaction_rating_id_idx ON rating_reaction (rating_id);

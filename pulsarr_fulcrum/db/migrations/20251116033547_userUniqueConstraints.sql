ALTER TABLE pulsarr_user
    ADD CONSTRAINT pulsarr_user_name_unique UNIQUE (name);
ALTER TABLE pulsarr_user
    ADD CONSTRAINT pulsarr_user_email_unique UNIQUE (email);
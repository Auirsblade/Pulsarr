-- User settings: profile visibility + crate visibility
ALTER TABLE pulsarr_user ADD COLUMN profile_visibility text NOT NULL DEFAULT 'public';
ALTER TABLE pulsarr_user ADD COLUMN crate_visibility text NOT NULL DEFAULT 'private';

-- Enforce one Personal group per user at DB level
CREATE UNIQUE INDEX unique_personal_group_per_user
    ON pulsarr_group (created_by_user_id)
    WHERE privacy_type = 'Personal';

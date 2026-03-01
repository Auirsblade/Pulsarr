CREATE INDEX idx_rating_user_id ON rating (pulsarr_user_id);
CREATE INDEX idx_rating_group_id ON rating (pulsarr_group_id);
CREATE INDEX idx_user_group_group_id ON user_group (pulsarr_group_id);
CREATE INDEX idx_user_group_user_id ON user_group (pulsarr_user_id);

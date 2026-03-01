-- First delete rating_details for duplicate ratings that will be removed
DELETE FROM rating_detail WHERE rating_id IN (
    SELECT r.rating_id FROM rating r
    WHERE r.rating_id NOT IN (
        SELECT DISTINCT ON (pulsarr_user_id, pulsarr_group_id, musicbrainz_id) rating_id
        FROM rating ORDER BY pulsarr_user_id, pulsarr_group_id, musicbrainz_id, rating_date DESC
    )
);

-- Then delete the duplicate ratings themselves (keep newest per user/group/media combo)
DELETE FROM rating WHERE rating_id NOT IN (
    SELECT DISTINCT ON (pulsarr_user_id, pulsarr_group_id, musicbrainz_id) rating_id
    FROM rating ORDER BY pulsarr_user_id, pulsarr_group_id, musicbrainz_id, rating_date DESC
);

-- Add unique constraint to prevent future duplicates
ALTER TABLE rating ADD CONSTRAINT rating_user_group_media_unique
    UNIQUE (pulsarr_user_id, pulsarr_group_id, musicbrainz_id);

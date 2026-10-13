-- Supports per-album aggregation within a group (group album rankings, group averages, by-album lookups).
CREATE INDEX IF NOT EXISTS idx_rating_group_musicbrainz ON rating (pulsarr_group_id, musicbrainz_id);

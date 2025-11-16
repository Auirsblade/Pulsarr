alter table rating
    add media_type text;
alter table rating
    add media_title text;
alter table rating
    add musicbrainz_id text;
alter table rating
    add artist_name text;
alter table rating
    add rating_date timestamp default CURRENT_TIMESTAMP;
alter table rating
    add release_date timestamp;
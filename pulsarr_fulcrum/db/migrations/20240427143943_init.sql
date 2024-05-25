-- Add migration script here
create table rating_system
(
    rating_system_id serial
        constraint RatingSystem_pk
            primary key,
    name text,
    master_rating_type text,
    rating_max numeric(8,3)
);

create table pulsarr_user
(
    pulsarr_user_id serial
        constraint user_pk
            primary key,
    name text
);

create table pulsarr_group
(
    pulsarr_group_id serial
        constraint group_pk
            primary key,
    rating_system_id integer not null
        constraint group_ratingSystem_ratingSystemId_fk
            references rating_system (rating_system_id),
    name text,
    privacy_type text
);

create table rating_system_parameter
(
    rating_system_parameter_id serial
        constraint rating_system_parameter_pk
            primary key,
    rating_system_id integer not null
        constraint ratingSystemParameter_ratingSystem_ratingSystemId_fk
            references rating_system (rating_system_id),
    name text,
    parameter_rating_max numeric(8,3)
);

create table rating
(
    rating_id serial
        constraint rating_pk
            primary key,
    pulsarr_user_id integer not null
        constraint rating_user_userId_fk
            references pulsarr_user (pulsarr_user_id),
    rating_system_id integer not null
        constraint rating_ratingSystem_ratingSystemId_fk
            references rating_system (rating_system_id),
    pulsarr_group_id integer not null
        constraint rating_group_groupId_fk
            references pulsarr_group (pulsarr_group_id),
    comments text,
    rating_value numeric(8,3)
);

create table rating_detail
(
    rating_detail_id serial
        constraint rating_detail_pk
            primary key,
    rating_id integer not null
        constraint ratingDetail_rating_ratingId_fk
            references rating (rating_id),
    rating_system_parameter_id integer not null
        constraint ratingDetail_ratingSystemParameter_ratingSystemParameterId_fk
            references rating_system_parameter (rating_system_parameter_id),
    rating_value numeric(8,3)
)
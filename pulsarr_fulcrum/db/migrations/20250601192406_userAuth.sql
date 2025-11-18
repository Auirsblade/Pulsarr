create table user_session
(
    user_session_uid uuid
        constraint user_session_pk
            primary key,
    pulsarr_user_id integer not null
        constraint session_user_userId_fk 
            references pulsarr_user (pulsarr_user_id) ON DELETE CASCADE
);

alter table pulsarr_user
    add column password text not null default 'invalid';

alter table pulsarr_user
    alter column password drop default;

alter table pulsarr_user
    add column email text not null default 'invalid';

alter table pulsarr_user
    alter column email drop default;

alter table pulsarr_user
    add column join_date timestamp not null default current_timestamp;
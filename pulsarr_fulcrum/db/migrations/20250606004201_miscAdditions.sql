create table user_group (
    user_group_id bigserial
        constraint user_group_pk
            primary key,
    pulsarr_user_id integer not null
        constraint user_group_user_id_fk
            references pulsarr_user (pulsarr_user_id) ON DELETE CASCADE,
    pulsarr_group_id integer not null
        constraint user_group_group_id_fk
            references pulsarr_group (pulsarr_group_id) ON DELETE CASCADE,
    join_date timestamp not null default current_timestamp,
    group_role text
);

alter table pulsarr_group
    add column creation_date timestamp not null default current_timestamp;

alter table pulsarr_group
    add column created_by_user_id integer
        constraint group_created_by_user_id_fk
            references pulsarr_user (pulsarr_user_id) ON DELETE SET NULL;

delete from user_session;

alter table user_session
    add column hw_key text not null;
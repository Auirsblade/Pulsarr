use sqlx::{query_as, FromRow, Postgres};
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use sqlx::types::chrono::NaiveDateTime;

#[derive(FromRow)]
pub struct UserGroup {
    pub user_group_id: i64,
    pub pulsarr_user_id: i32,
    pub pulsarr_group_id: i32,
    pub group_role: String,
    pub join_date: NaiveDateTime,
}

pub(crate) fn join(group_id: i32, user_id: i32, role: String) -> QueryAs<'static, Postgres, UserGroup, PgArguments>{
    query_as::<Postgres, UserGroup>("
        INSERT INTO user_group (pulsarr_group_id, pulsarr_user_id, group_role) \
        VALUES ($1, $2, $3) \
        ")
        .bind(group_id)
        .bind(user_id)
        .bind(role)
}

pub(crate) fn leave(group_id: i32, user_id: i32) -> QueryAs<'static, Postgres, UserGroup, PgArguments>{
    query_as::<Postgres, UserGroup>("DELETE FROM user_group WHERE pulsarr_group_id = $1 AND pulsarr_user_id = $2")
        .bind(group_id)
        .bind(user_id)
}

pub(crate) fn get_by_group_id(group_id: i32) -> QueryAs<'static, Postgres, UserGroup, PgArguments>{
    query_as::<Postgres, UserGroup>("SELECT * FROM user_group WHERE pulsarr_group_id = $1")
        .bind(group_id)
}

pub(crate) fn get_by_user_id(user_id: i32) -> QueryAs<'static, Postgres, UserGroup, PgArguments>{
    query_as::<Postgres, UserGroup>("SELECT * FROM user_group WHERE pulsarr_user_id = $1")
        .bind(user_id)
}
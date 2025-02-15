use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx;
use sqlx::{FromRow, Postgres, query_as};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct PulsarrUser {
    pulsarr_user_id: i32,
    name: String,
}

impl Model for PulsarrUser {
    fn add<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        sqlx::query_as(
            "INSERT INTO pulsarr_user (name)\
            VALUES ($1)\
            RETURNING *",
        )
        .bind(self.name)
    }

    fn update<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        sqlx::query_as(
            "UPDATE pulsarr_user \
             SET name = $2 \
            WHERE pulsarr_user_id = $1 \
            RETURNING *",
        )
        .bind(self.pulsarr_user_id)
        .bind(self.name)
    }

    fn delete<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        query_as("DELETE FROM pulsarr_user WHERE pulsarr_user_id = $1")
            .bind(id)
    }

    fn get_by_id<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        query_as("SELECT * FROM pulsarr_user WHERE pulsarr_user_id = $1")
            .bind(id)
    }

    fn get_all<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>() -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        query_as("SELECT * FROM pulsarr_user")
    }
}
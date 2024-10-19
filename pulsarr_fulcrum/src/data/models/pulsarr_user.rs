use crate::data::data_wrangler;
use crate::error::PulsarrError;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket::futures::stream::Select;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
    JsonSchema,
};
use sqlx;
use sqlx::{FromRow, PgPool, Postgres, query_as};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use crate::data::models::Model;
use crate::PostgresState;

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
            "UPDATE pulsarr_user\
             SET name = $2\
            WHERE pulsarr_user_id = $1\
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
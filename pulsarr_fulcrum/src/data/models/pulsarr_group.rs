use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use sqlx::{Arguments, FromRow, PgPool, Postgres, query_as};
use sqlx::database::HasArguments;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::{Query, QueryAs};
use crate::data::models::Model;

use crate::error::PulsarrError;
use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct PulsarrGroup {
    pulsarr_group_id: i32,
    rating_system_id: i32,
    name: String,
    privacy_type: String,
}

pub(crate) const PRIVACY_TYPE: [&str; 2] = ["Public", "Private"];

impl Model for PulsarrGroup {
    fn add<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as(
            "INSERT INTO pulsarr_group (rating_system_id, name, privacy_type)\
                VALUES ($1, $2, $3)"
        )
        .bind(self.rating_system_id)
        .bind(self.name)
        .bind(self.privacy_type)
    }

    // fn update<PulsarrGroup>(self) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
    //     query_as(
    //         "INSERT INTO pulsarr_group (pulsarr_group_id, rating_system_id, name, privacy_type)\
    //     VALUES ($1, $2, $3, $4)",
    //     )
    //         .bind(self.pulsarr_group_id)
    //         .bind(self.rating_system_id)
    //         .bind(self.name)
    //         .bind(self.privacy_type)
    // }
    //
    // fn delete<PulsarrGroup>(id: i32) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
    //     query_as("DELETE FROM pulsarr_group WHERE pulsarr_group_id = $1")
    //         .bind(id)
    // }

    fn get_by_id<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as("SELECT FROM pulsarr_group WHERE pulsarr_group_id = $1")
            .bind(id)
    }
    //
    // fn get_all<PulsarrGroup>() -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
    //     query_as("SELECT FROM pulsarr_group")
    // }
}
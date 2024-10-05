use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use sqlx::{FromRow, PgPool};

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

async fn new_save(group: PulsarrGroup, pool: &PgPool) -> (bool, Option<String>) {
    let result = sqlx::query(
        "INSERT INTO pulsarr_group (rating_system_id, name, privacy_type)\
        VALUES ($1, $2, $3)",
    )
        .bind(group.rating_system_id)
        .bind(group.name)
        .bind(group.privacy_type)
        .execute(pool)
        .await;

    match result {
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string()))
    }
}

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
    JsonSchema,
};
use sqlx;
use sqlx::types::Decimal;
use sqlx::FromRow;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystem {
    rating_system_id: i32,
    master_rating_type: String,
    rating_max: Decimal,
    name: String,
}

pub(crate) const RATING_TYPE: [&str; 3] = ["Absolute", "Cumulative", "Average"];

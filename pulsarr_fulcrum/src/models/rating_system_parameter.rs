use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::FromRow;
use sqlx::types::Decimal;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct RatingSystemParameter {
    rating_system_parameter_id: i32,
    rating_system_id: i32,
    parameter_rating_max: Decimal,
    name: String,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system_parameter]
}

/// # Get a rating system parameter by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system_parameter(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystemParameter> {
    let rating_system = sqlx::query_as::<_, RatingSystemParameter>(
        "select * from rating_system_parameter where rating_system_parameter_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(rating_system))
}

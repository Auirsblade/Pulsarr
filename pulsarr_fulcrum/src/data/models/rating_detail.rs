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
struct RatingDetail {
    rating_detail_id: i32,
    rating_id: i32,
    rating_system_parameter_id: i32,
    rating_value: Decimal,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_detail]
}

/// # Get a rating detail by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating_detail(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingDetail> {
    let group = sqlx::query_as::<_, RatingDetail>(
        "select * from rating_detail where rating_detail_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(group))
}

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
struct RatingSystem {
    rating_system_id: i32,
    master_rating_type: String,
    rating_max: Decimal,
    name: String,
}

const RATING_TYPE: [&str; 3] = ["Absolute", "Cumulative", "Average"];

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system, get_rating_types]
}

/// # Get a rating system by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystem> {
    let rating_system = sqlx::query_as::<_, RatingSystem>(
        "select * from rating_system where rating_system_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(rating_system))
}

/// # Get the master rating types
#[openapi(tag = "Rating System")]
#[get("/ratingTypes")]
async fn get_rating_types() -> crate::PulsarrResult<Vec<String>> {
    let mut rating_types = vec![];

    for typ in RATING_TYPE {
        rating_types.push(typ.to_owned());
    }

    Ok(Json(rating_types))
}

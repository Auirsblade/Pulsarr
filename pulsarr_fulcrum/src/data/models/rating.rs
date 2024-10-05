use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::{FromRow, types::Decimal};

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct Rating {
    rating_id: i32,
    pulsarr_user_id: i32,
    pulsarr_group_id: i32,
    rating_system_id: i32,
    comments: String,
    rating_value: Decimal,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating]
}

/// # Get a rating by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<Rating> {
    let group = sqlx::query_as::<_, Rating>("select * from rating where rating_id = $1")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(Json(group))
}

use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::{
    JsonSchema, okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec,
    settings::OpenApiSettings,
};
use sqlx;
use sqlx::FromRow;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct PulsarrUser {
    pulsarr_user_id: i32,
    name: String,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_pulsarr_user]
}

/// # Get a user by id
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_pulsarr_user(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<PulsarrUser> {
    let rating_system =
        sqlx::query_as::<_, PulsarrUser>("select * from pulsarr_user where pulsarr_user_id = $1")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .unwrap();

    Ok(Json(rating_system))
}

use std::result;
use crate::error::PulsarrError;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
    JsonSchema,
};
use sqlx;
use sqlx::{FromRow, PgPool};

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct PulsarrUser {
    pulsarr_user_id: i32,
    name: String,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_pulsarr_user, add_user]
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

/// # Add a user
#[openapi(tag = "User")]
#[post("/", format = "application/json", data = "<user>")]
async fn add_user(
    state: &State<PostgresState>,
    user: Json<PulsarrUser>,
) -> crate::PulsarrResult<bool> {
    match new_save(user.into_inner(), &state.pool).await {
        (true, _) => Ok(Json(true)),
        (false, error_message) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: error_message,
            http_status_code: 400,
        }),
    }
}

async fn new_save(user: PulsarrUser, pool: &PgPool) -> (bool, Option<String>) {
    let result = sqlx::query(
        "INSERT INTO pulsarr_user (name)\
        VALUES ($1)",
    )
    .bind(user.name)
    .execute(pool)
    .await;

    match result { 
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string()))
    }
}

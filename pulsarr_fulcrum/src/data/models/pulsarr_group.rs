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
struct PulsarrGroup {
    pulsarr_group_id: i32,
    rating_system_id: i32,
    name: String,
    privacy_type: String,
}

const PRIVACY_TYPE: [&str; 2] = ["Public", "Private"];

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_pulsarr_group, get_privacy_types, add_group]
}

/// # Get a group by id
#[openapi(tag = "Group")]
#[get("/<id>")]
async fn get_pulsarr_group(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<PulsarrGroup> {
    let group = sqlx::query_as::<_, PulsarrGroup>(
        "select * from pulsarr_group where pulsarr_group_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(group))
}

/// # Get the group privacy types
#[openapi(tag = "Group")]
#[get("/privacyTypes")]
async fn get_privacy_types() -> crate::PulsarrResult<Vec<String>> {
    let mut privacy_types = vec![];

    for typ in PRIVACY_TYPE {
        privacy_types.push(typ.to_owned());
    }

    Ok(Json(privacy_types))
}

/// # Add a group
#[openapi(tag = "Group")]
#[post("/", format = "application/json", data = "<group>")]
async fn add_group(
    state: &State<PostgresState>,
    group: Json<PulsarrGroup>,
) -> crate::PulsarrResult<bool> {
    match new_save(group.into_inner(), &state.pool).await {
        (true, _) => Ok(Json(true)),
        (false, error_message) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: error_message,
            http_status_code: 400
        })
    }
}

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

use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::FromRow;

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
    openapi_get_routes_spec![settings: get_pulsarr_group, get_privacy_types]
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

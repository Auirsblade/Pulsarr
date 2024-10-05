use crate::data::data_wrangler;
use crate::data::models::pulsarr_group::{PulsarrGroup, PRIVACY_TYPE};
use crate::error::PulsarrError;
use crate::PostgresState;
use rocket::serde::json::Json;
use rocket::{get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use sqlx::PgPool;

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
) -> crate::PulsarrResult<PulsarrGroup> {
    data_wrangler::add(group.into_inner(), &state.pool).await
}

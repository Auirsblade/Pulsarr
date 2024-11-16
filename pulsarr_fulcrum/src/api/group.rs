use crate::data::data_wrangler;
use crate::data::models::pulsarr_group::{PulsarrGroup, PRIVACY_TYPE};
use crate::error::PulsarrError;
use crate::{PostgresState, PulsarrResult};
use rocket::serde::json::Json;
use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use sqlx::PgPool;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_group, update_group, delete_group, get_pulsarr_group, get_all_groups, get_privacy_types, ]
}

/// # Get the group privacy types
#[openapi(tag = "Group")]
#[get("/privacyTypes")]
async fn get_privacy_types() -> PulsarrResult<Vec<String>> {
    let mut privacy_types = vec![];

    for typ in PRIVACY_TYPE {
        privacy_types.push(typ.to_owned());
    }

    Ok(Json(privacy_types))
}

/// # Add a group
#[openapi(tag = "Group")]
#[post("/add", format = "application/json", data = "<group>")]
async fn add_group(state: &State<PostgresState>, group: Json<PulsarrGroup>) -> PulsarrResult<PulsarrGroup> {
    data_wrangler::add(group.into_inner(), &state.pool).await
}

/// # Update group
#[openapi(tag = "Group")]
#[post("/update", format = "application/json", data = "<group>")]
async fn update_group(state: &State<PostgresState>, group: Json<PulsarrGroup>) -> PulsarrResult<PulsarrGroup> {
    data_wrangler::update(group.into_inner(), &state.pool).await
}

/// # Delete group
#[openapi(tag = "Group")]
#[delete("/delete/<id>")]
async fn delete_group(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool> {
    data_wrangler::delete::<PulsarrGroup>(id, &state.pool).await
}

/// # Get a group by id
#[openapi(tag = "Group")]
#[get("/<id>")]
async fn get_pulsarr_group(state: &State<PostgresState>, id: i32) -> PulsarrResult<PulsarrGroup> {
    data_wrangler::get_by_id::<PulsarrGroup>(id, &state.pool).await
}

/// # Get all groups
#[openapi(tag = "Group")]
#[get("/")]
async fn get_all_groups(state: &State<PostgresState>) -> PulsarrResult<Vec<PulsarrGroup>> {
    data_wrangler::get_all::<PulsarrGroup>(&state.pool).await
}
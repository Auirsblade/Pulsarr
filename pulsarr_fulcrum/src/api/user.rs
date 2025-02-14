use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::PostgresState;
use crate::data::{data_wrangler, models::pulsarr_user::PulsarrUser};

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_user, update_user, delete_user, get_pulsarr_user, get_all_users]
}

/// # Add user
#[openapi(tag = "User")]
#[post("/add", format = "application/json", data = "<user>")]
async fn add_user(state: &State<PostgresState>, user: Json<PulsarrUser>) -> crate::PulsarrResult<PulsarrUser> {
    data_wrangler::add(user.into_inner(), &state.pool).await
}

/// # Update user
#[openapi(tag = "User")]
#[post("/update", format = "application/json", data = "<user>")]
async fn update_user(state: &State<PostgresState>, user: Json<PulsarrUser>) -> crate::PulsarrResult<PulsarrUser> {
    data_wrangler::update(user.into_inner(), &state.pool).await
}

/// # Delete user
#[openapi(tag = "User")]
#[delete("/delete/<id>")]
async fn delete_user(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<bool> {
    data_wrangler::delete::<PulsarrUser>(id, &state.pool).await
}

/// # Get a user by id
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_pulsarr_user(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<PulsarrUser> {
    data_wrangler::get_by_id::<PulsarrUser>(id, &state.pool).await
}

/// # Get all users
#[openapi(tag = "User")]
#[get("/")]
async fn get_all_users(state: &State<PostgresState>) -> crate::PulsarrResult<Vec<PulsarrUser>> {
    data_wrangler::get_all::<PulsarrUser>(&state.pool).await
}
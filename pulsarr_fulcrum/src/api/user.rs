use crate::data::{data_wrangler, models::pulsarr_user::PulsarrUser};
use crate::PostgresState;
use rocket::serde::json::Json;
use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use crate::api::guards::api_key::ApiKey;
use crate::api::dtos::user_dto::*;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_user, update_user, delete_user, get_pulsarr_user, get_current_user, get_all_users]
}

/// # Add user
#[openapi(tag = "User")]
#[post("/add", format = "application/json", data = "<user>")]
async fn add_user(state: &State<PostgresState>, user: Json<UserDTO>) -> crate::PulsarrResult<UserDTO> {
    let pulsarr_user = to_model(user);
    match data_wrangler::add(pulsarr_user, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(to_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Update user
#[openapi(tag = "User")]
#[post("/update", format = "application/json", data = "<user>")]
async fn update_user(state: &State<PostgresState>, user: Json<UserDTO>) -> crate::PulsarrResult<UserDTO> {
    let pulsarr_user = to_model(user);
    match data_wrangler::update(pulsarr_user, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(to_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Delete user
#[openapi(tag = "User")]
#[delete("/delete/<id>")]
async fn delete_user(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<bool> {
    match data_wrangler::delete::<PulsarrUser>(id, &state.pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(error),
    }
}

/// # Get a user by id
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_pulsarr_user(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<UserDTO> {
    match data_wrangler::get_by_id::<PulsarrUser>(id, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(to_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Get a user by active session
#[openapi(tag = "User")]
#[get("/currentUser")]
async fn get_current_user(state: &State<PostgresState>, api_user: ApiKey) -> crate::PulsarrResult<UserDTO> {
    let ApiKey(user_id) = api_user;
    match data_wrangler::get_by_id::<PulsarrUser>(user_id, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(to_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Get all users
#[openapi(tag = "User")]
#[get("/")]
async fn get_all_users(state: &State<PostgresState>, _api_user: ApiKey) -> crate::PulsarrResult<Vec<UserDTO>> {
    match data_wrangler::get_all::<PulsarrUser>(&state.pool, None).await {
        Ok(pulsarr_user) =>
            Ok(Json(pulsarr_user.iter().map(|user| to_dto(user)).collect::<Vec<UserDTO>>())),
        Err(error) => Err(error),
    }
}
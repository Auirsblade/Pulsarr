use crate::data::models::pulsarr_user;
use crate::data::{data_wrangler, models::pulsarr_user::PulsarrUser};
use crate::error::PulsarrError;
use crate::PostgresState;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use schemars::JsonSchema;
use serde::Serialize;
use sqlx::FromRow;
use crate::api::structs::api_key::ApiKey;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_user, update_user, delete_user, get_pulsarr_user, get_current_user, get_all_users]
}

/// # Add user
#[openapi(tag = "User")]
#[post("/add", format = "application/json", data = "<user>")]
async fn add_user(state: &State<PostgresState>, user: Json<UserDTO>) -> crate::PulsarrResult<UserDTO> {
    let pulsarr_user = map_pulsarr_user(user);
    match data_wrangler::add(pulsarr_user, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(map_user_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Update user
#[openapi(tag = "User")]
#[post("/update", format = "application/json", data = "<user>")]
async fn update_user(state: &State<PostgresState>, user: Json<UserDTO>) -> crate::PulsarrResult<UserDTO> {
    let pulsarr_user = map_pulsarr_user(user);
    match data_wrangler::update(pulsarr_user, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(map_user_dto(&pulsarr_user))),
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
        Ok(pulsarr_user) => Ok(Json(map_user_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Get a user by active session
#[openapi(tag = "User")]
#[get("/currentUser")]
async fn get_current_user(state: &State<PostgresState>, api_user: ApiKey) -> crate::PulsarrResult<UserDTO> {
    let ApiKey(user_id) = api_user;
    match data_wrangler::get_by_id::<PulsarrUser>(user_id, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(map_user_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Get all users
#[openapi(tag = "User")]
#[get("/")]
async fn get_all_users(state: &State<PostgresState>, user_id: ApiKey) -> crate::PulsarrResult<Vec<UserDTO>> {
    match data_wrangler::get_all::<PulsarrUser>(&state.pool).await {
        Ok(pulsarr_user) =>
            Ok(Json(pulsarr_user.iter().map(|user| map_user_dto(user)).collect::<Vec<UserDTO>>())),
        Err(error) => Err(error),
    }
}


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserDTO {
    pub pulsarr_user_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

fn map_user_dto(pulsarr_user: &PulsarrUser) -> UserDTO {
    UserDTO {
        pulsarr_user_id: pulsarr_user.pulsarr_user_id,
        name: pulsarr_user.name.clone(),
        email: pulsarr_user.email.clone(),
        password: "".to_string()
    }
}

fn map_pulsarr_user(user: Json<UserDTO>) -> PulsarrUser {
    PulsarrUser {
        pulsarr_user_id: user.pulsarr_user_id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone()
    }
}
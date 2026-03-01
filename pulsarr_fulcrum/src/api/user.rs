use crate::data::{data_wrangler, models::pulsarr_user::{self, PulsarrUser}};
use crate::error::PulsarrError;
use crate::PostgresState;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use scrypt::password_hash::{PasswordHash, PasswordVerifier};
use scrypt::Scrypt;
use crate::api::guards::api_key::ApiKey;
use crate::api::dtos::user_dto::*;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_user, update_user, delete_user, get_pulsarr_user, get_current_user, get_all_users, change_password]
}

fn validate_username(name: &str) -> Result<(), PulsarrError> {
    if name.len() < 3 || name.len() > 30 {
        return Err(PulsarrError::validation_error("Username must be between 3 and 30 characters"));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(PulsarrError::validation_error("Username may only contain letters, numbers, and underscores"));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), PulsarrError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(PulsarrError::validation_error("Invalid email format"));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), PulsarrError> {
    if password.len() < 8 {
        return Err(PulsarrError::validation_error("Password must be at least 8 characters"));
    }
    Ok(())
}

/// # Add user
#[openapi(tag = "User")]
#[post("/add", format = "application/json", data = "<user>")]
async fn add_user(state: &State<PostgresState>, user: Json<CreateUserRequest>) -> crate::PulsarrResult<UserDTO> {
    let req = user.into_inner();
    validate_username(&req.name)?;
    validate_email(&req.email)?;
    validate_password(&req.password)?;

    let pulsarr_user = create_request_to_model(req);
    match data_wrangler::add(pulsarr_user, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(to_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Update user
#[openapi(tag = "User")]
#[post("/update", format = "application/json", data = "<user>")]
async fn update_user(state: &State<PostgresState>, user: Json<UserDTO>, api_user: ApiKey) -> crate::PulsarrResult<UserDTO> {
    let ApiKey(caller_id) = api_user;
    let dto = user.into_inner();

    if dto.pulsarr_user_id != caller_id {
        return Err(PulsarrError::forbidden("Cannot update another user's profile"));
    }

    let pulsarr_user = to_model(dto);
    match data_wrangler::update(pulsarr_user, &state.pool).await {
        Ok(pulsarr_user) => Ok(Json(to_dto(&pulsarr_user))),
        Err(error) => Err(error),
    }
}

/// # Delete user
#[openapi(tag = "User")]
#[delete("/delete/<id>")]
async fn delete_user(state: &State<PostgresState>, id: i32, api_user: ApiKey) -> crate::PulsarrResult<bool> {
    let ApiKey(caller_id) = api_user;
    if caller_id != id {
        return Err(PulsarrError::forbidden("Cannot delete another user's account"));
    }

    match data_wrangler::delete::<PulsarrUser>(id, &state.pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(error),
    }
}

/// # Get a user by id
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_pulsarr_user(state: &State<PostgresState>, id: i32, _api_user: ApiKey) -> crate::PulsarrResult<UserDTO> {
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
    match data_wrangler::get_all::<PulsarrUser>(&state.pool, None, None).await {
        Ok(pulsarr_user) =>
            Ok(Json(pulsarr_user.iter().map(|user| to_dto(user)).collect::<Vec<UserDTO>>())),
        Err(error) => Err(error),
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct ChangePasswordRequest {
    current_password: String,
    new_password: String,
}

/// # Change password
#[openapi(tag = "User")]
#[post("/change-password", format = "application/json", data = "<request>")]
async fn change_password(
    state: &State<PostgresState>,
    api_user: ApiKey,
    request: Json<ChangePasswordRequest>,
) -> crate::PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    let req = request.into_inner();

    validate_password(&req.new_password)?;

    let user = match data_wrangler::get_by_id::<PulsarrUser>(user_id, &state.pool).await {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(h) => h,
        Err(_) => return Err(PulsarrError::validation_error("Failed to parse stored password hash")),
    };

    if Scrypt.verify_password(req.current_password.as_ref(), &parsed_hash).is_err() {
        return Err(PulsarrError {
            err: "Incorrect password".to_owned(),
            msg: Some("Current password is incorrect".to_owned()),
            http_status_code: 400,
        });
    }

    let new_hash = pulsarr_user::hash_password(req.new_password);
    match pulsarr_user::update_password::<PulsarrUser>(user_id, new_hash)
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::internal_error("Failed to update password", e)),
    }
}

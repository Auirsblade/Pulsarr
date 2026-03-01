use scrypt::{ password_hash::{ PasswordHash }, Scrypt };
use rocket::{post, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use rocket_okapi::settings::OpenApiSettings;
use scrypt::password_hash::PasswordVerifier;
use uuid::Uuid;
use crate::{PostgresState, PulsarrResult};
use crate::api::dtos::user_dto::*;
use crate::data::models::{pulsarr_user, user_session};
use crate::data::models::pulsarr_user::PulsarrUser;
use crate::error::PulsarrError;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: signin]
}

/// # Sign in
#[openapi(tag = "Auth")]
#[post("/signin", format = "application/json", data = "<request>")]
async fn signin(state: &State<PostgresState>, request: Json<SignInRequest>) -> PulsarrResult<SignInResponse> {
    if request.username.is_empty() || request.password.is_empty() {
        return Err(PulsarrError::validation_error("Username and password are required"));
    }

    match pulsarr_user::get_password_hash::<PulsarrUser>(&request.username).fetch_one(&state.pool).await
    {
        Ok(user) => {
            let parsed_hash = match PasswordHash::new(&user.password) {
                Ok(h) => h,
                Err(_) => return Err(PulsarrError {
                    err: "Invalid username or password".to_owned(),
                    msg: None,
                    http_status_code: 401,
                }),
            };
            match Scrypt.verify_password(request.password.as_ref(), &parsed_hash).is_ok() {
                true => {
                    let session_id = Uuid::new_v4();
                    match user_session::start_session(session_id, &user.pulsarr_user_id, &request.into_inner().hw_key, &state.pool).await {
                        Ok(result) => Ok(Json(SignInResponse {
                            pulsarr_api_key: result.user_session_uid.to_string(),
                            user: to_dto(&user)
                        })),
                        Err(e) => Err(PulsarrError::internal_error("Failed to start session", e)),
                    }
                },
                false => Err(PulsarrError {
                    err: "Invalid username or password".to_owned(),
                    msg: None,
                    http_status_code: 401,
                })
            }
        },
        Err(_) => Err(PulsarrError {
            err: "Invalid username or password".to_owned(),
            msg: None,
            http_status_code: 401,
        })
    }
}

/// Request/response guards
#[derive(Deserialize, Serialize, JsonSchema)]
struct SignInRequest {
    username: String,
    password: String,
    hw_key: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct SignInResponse {
    pulsarr_api_key: String,
    user: UserDTO
}

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
    match pulsarr_user::get_password_hash::<PulsarrUser>(&request.username).fetch_one(&state.pool).await    
    {
        Ok(user) => {
            println!("getting password hash from db");
            let parsed_hash = PasswordHash::new(&user.password).unwrap();
            println!("verifying match");
            match Scrypt.verify_password(request.password.as_ref(), &parsed_hash).is_ok() {
                true => {
                    println!("starting session");   
                    let session_id = Uuid::new_v4();
                    match user_session::start_session(session_id, &user.pulsarr_user_id, &request.into_inner().hw_key, &state.pool).await {
                        Ok(result) => Ok(Json(SignInResponse {
                            pulsarr_api_key: result.user_session_uid.to_string(),
                            user: to_dto(&user)
                        })),
                        Err(error) => Err(PulsarrError {
                            err: "Failed to start session".to_owned(),
                            msg: Some(error.to_string()),
                            http_status_code: 500,
                        }),
                    }
                },
                false => Err(PulsarrError {
                    err: "Incorrect password".to_owned(),
                    msg: None,
                    http_status_code: 400,
                })   
            }
        },
        Err(error) => Err(PulsarrError {
            err: "User not found".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 404,
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
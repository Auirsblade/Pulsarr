use scrypt::{ password_hash::{ PasswordHash }, Scrypt };
use rocket::{post, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use rocket_okapi::settings::OpenApiSettings;
use scrypt::password_hash::PasswordVerifier;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use crate::{PostgresState, EmailState, PulsarrResult};
use crate::api::dtos::user_dto::*;
use crate::data::models::{pulsarr_user, user_session};
use crate::data::models::pulsarr_user::PulsarrUser;
use crate::error::PulsarrError;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: signin, forgot_password, validate_reset_token, reset_password]
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

/// # Forgot password — request a reset email
#[openapi(tag = "Auth")]
#[post("/forgot-password", format = "application/json", data = "<request>")]
async fn forgot_password(
    state: &State<PostgresState>,
    email_state: &State<EmailState>,
    request: Json<ForgotPasswordRequest>,
) -> PulsarrResult<GenericResponse> {
    let email = request.into_inner().email;

    // Always return success to prevent email enumeration
    let success = Json(GenericResponse {
        message: "If an account with that email exists, a reset link has been sent.".to_string(),
    });

    // Look up user
    let user = match pulsarr_user::get_by_email::<PulsarrUser>(&email).fetch_optional(&state.pool).await {
        Ok(Some(u)) => u,
        _ => return Ok(success),
    };

    // Delete existing tokens for this user + expired tokens
    let _ = sqlx::query("DELETE FROM password_reset_token WHERE pulsarr_user_id = $1 OR expires_at < now()")
        .bind(user.pulsarr_user_id)
        .execute(&state.pool)
        .await;

    // Generate token (scoped to drop rng before awaits)
    let raw_token = {
        use rand::Rng;
        let mut rng = rand::rng();
        let token_bytes: [u8; 32] = rng.random();
        hex::encode(token_bytes)
    };

    // Hash token for storage
    let token_hash = hex::encode(Sha256::digest(raw_token.as_bytes()));

    // Store hashed token
    let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(1);
    let _ = sqlx::query(
        "INSERT INTO password_reset_token (token_hash, pulsarr_user_id, expires_at) VALUES ($1, $2, $3)"
    )
        .bind(&token_hash)
        .bind(user.pulsarr_user_id)
        .bind(expires_at)
        .execute(&state.pool)
        .await;

    // Send email (raw token, not hash)
    if let Err(e) = email_state.sender.send_password_reset(&email, &raw_token) {
        eprintln!("Failed to send password reset email: {}", e);
    }

    Ok(success)
}

/// # Validate a password reset token
#[openapi(tag = "Auth")]
#[post("/validate-reset-token", format = "application/json", data = "<request>")]
async fn validate_reset_token(
    state: &State<PostgresState>,
    request: Json<ValidateTokenRequest>,
) -> PulsarrResult<ValidateTokenResponse> {
    let token_hash = hex::encode(Sha256::digest(request.token.as_bytes()));

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM password_reset_token WHERE token_hash = $1 AND used = false AND expires_at > now())"
    )
        .bind(&token_hash)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(false);

    Ok(Json(ValidateTokenResponse { valid: exists }))
}

/// # Reset password using a valid token
#[openapi(tag = "Auth")]
#[post("/reset-password", format = "application/json", data = "<request>")]
async fn reset_password(
    state: &State<PostgresState>,
    request: Json<ResetPasswordRequest>,
) -> PulsarrResult<GenericResponse> {
    let req = request.into_inner();

    if req.new_password.len() < 8 {
        return Err(PulsarrError::validation_error("Password must be at least 8 characters"));
    }

    let token_hash = hex::encode(Sha256::digest(req.token.as_bytes()));

    // Look up token
    #[derive(sqlx::FromRow)]
    struct TokenRow {
        pulsarr_user_id: i32,
    }

    let token_row = sqlx::query_as::<_, TokenRow>(
        "SELECT pulsarr_user_id FROM password_reset_token WHERE token_hash = $1 AND used = false AND expires_at > now()"
    )
        .bind(&token_hash)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| PulsarrError::internal_error("Failed to validate token", e))?;

    let token_row = match token_row {
        Some(t) => t,
        None => return Err(PulsarrError::validation_error("Invalid or expired reset token")),
    };

    // Hash new password and update
    let password_hash = pulsarr_user::hash_password(req.new_password);
    pulsarr_user::update_password::<PulsarrUser>(token_row.pulsarr_user_id, password_hash)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| PulsarrError::internal_error("Failed to update password", e))?;

    // Mark token as used
    let _ = sqlx::query("UPDATE password_reset_token SET used = true WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(&state.pool)
        .await;

    // Invalidate all sessions for this user
    let _ = sqlx::query("DELETE FROM user_session WHERE pulsarr_user_id = $1")
        .bind(token_row.pulsarr_user_id)
        .execute(&state.pool)
        .await;

    Ok(Json(GenericResponse {
        message: "Password has been reset successfully.".to_string(),
    }))
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

#[derive(Deserialize, Serialize, JsonSchema)]
struct ForgotPasswordRequest {
    email: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct GenericResponse {
    message: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct ValidateTokenRequest {
    token: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct ValidateTokenResponse {
    valid: bool,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct ResetPasswordRequest {
    token: String,
    new_password: String,
}

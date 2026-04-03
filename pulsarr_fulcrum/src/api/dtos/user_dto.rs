use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::types::chrono::NaiveDateTime;
use crate::data::models::pulsarr_user::PulsarrUser;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserDTO {
    pub pulsarr_user_id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_visibility: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn to_dto(pulsarr_user: &PulsarrUser) -> UserDTO {
    UserDTO {
        pulsarr_user_id: pulsarr_user.pulsarr_user_id,
        name: pulsarr_user.name.clone(),
        email: pulsarr_user.email.clone(),
        join_date: Some(pulsarr_user.join_date),
        profile_visibility: Some(pulsarr_user.profile_visibility.clone()),
        crate_visibility: Some(pulsarr_user.crate_visibility.clone()),
    }
}

pub fn create_request_to_model(req: CreateUserRequest) -> PulsarrUser {
    PulsarrUser {
        pulsarr_user_id: 0,
        name: req.name,
        email: req.email,
        password: req.password,
        join_date: chrono::Local::now().naive_local(),
        profile_visibility: "public".to_string(),
        crate_visibility: "private".to_string(),
    }
}

pub fn to_model(user: UserDTO) -> PulsarrUser {
    PulsarrUser {
        pulsarr_user_id: user.pulsarr_user_id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: "".to_string(),
        join_date: user.join_date.unwrap_or_else(|| chrono::Local::now().naive_local()),
        profile_visibility: user.profile_visibility.unwrap_or_else(|| "public".to_string()),
        crate_visibility: user.crate_visibility.unwrap_or_else(|| "private".to_string()),
    }
}

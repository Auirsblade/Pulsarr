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
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_date: Option<NaiveDateTime>
}

pub fn to_dto(pulsarr_user: &PulsarrUser) -> UserDTO {
    UserDTO {
        pulsarr_user_id: pulsarr_user.pulsarr_user_id,
        name: pulsarr_user.name.clone(),
        email: pulsarr_user.email.clone(),
        password: None,
        join_date: Some(pulsarr_user.join_date)
    }
}

pub fn to_model(user: UserDTO) -> PulsarrUser {
    PulsarrUser {
        pulsarr_user_id: user.pulsarr_user_id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.unwrap_or_else(|| "".to_string()),
        join_date: user.join_date.unwrap_or_else(|| chrono::Local::now().naive_local()),
    }
}
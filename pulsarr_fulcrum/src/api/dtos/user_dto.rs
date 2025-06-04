use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::JsonSchema;
use crate::data::models::pulsarr_user::PulsarrUser;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserDTO {
    pub pulsarr_user_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn to_dto(pulsarr_user: &PulsarrUser) -> UserDTO {
    UserDTO {
        pulsarr_user_id: pulsarr_user.pulsarr_user_id,
        name: pulsarr_user.name.clone(),
        email: pulsarr_user.email.clone(),
        password: "".to_string()
    }
}

pub fn to_model(user: Json<UserDTO>) -> PulsarrUser {
    PulsarrUser {
        pulsarr_user_id: user.pulsarr_user_id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone()
    }
}
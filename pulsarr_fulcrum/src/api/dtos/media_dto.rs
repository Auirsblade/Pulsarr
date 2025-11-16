use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use crate::data::models::pulsarr_user::PulsarrUser;
use crate::musicbrainz_client::Artist;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MediaDTO {
    pub media_id: i32,
    pub album_name: String,
    pub artist: Artist,
    pub release_date: NaiveDateTime,
    pub tracks: Vec<String>
}


//
// pub fn to_dto(pulsarr_user: &PulsarrUser) -> crate::api::dtos::user_dto::UserDTO {
//     crate::api::dtos::user_dto::UserDTO {
//         pulsarr_user_id: pulsarr_user.pulsarr_user_id,
//         name: pulsarr_user.name.clone(),
//         email: pulsarr_user.email.clone(),
//         password: "".to_string(),
//         join_date: Some(pulsarr_user.join_date)
//     }
// }
//
// pub fn to_model(user: crate::api::dtos::user_dto::UserDTO) -> PulsarrUser {
//     PulsarrUser {
//         pulsarr_user_id: user.pulsarr_user_id,
//         name: user.name.clone(),
//         email: user.email.clone(),
//         password: user.password.clone(),
//         join_date: user.join_date.unwrap_or_else(|| chrono::Local::now().naive_local()),
//     }
// }
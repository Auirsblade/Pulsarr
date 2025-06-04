use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use crate::api::dtos::rating_system_dto;
use crate::api::dtos::rating_system_dto::RatingSystemDTO;
use crate::data::models::pulsarr_group::PulsarrGroup;
use crate::data::models::rating_system::RatingSystem;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GroupDTO {
    pub pulsarr_group_id: i32,
    pub rating_system_id: i32,
    pub name: String,
    pub privacy_type: String,
    pub rating_system: Option<RatingSystemDTO>,
}

pub fn to_dto(pulsarr_group: PulsarrGroup, rating_system: Option<RatingSystem>) -> GroupDTO {
    GroupDTO{
        pulsarr_group_id: pulsarr_group.pulsarr_group_id,
        rating_system_id: pulsarr_group.rating_system_id,
        name: pulsarr_group.name,
        privacy_type: pulsarr_group.privacy_type,
        rating_system: match rating_system { 
            Some(rs) => Some(rating_system_dto::to_dto(rs)), 
            None => None
        }
    }
}

pub fn to_model(pulsarr_group_dto: &GroupDTO) -> PulsarrGroup {
    PulsarrGroup{
        pulsarr_group_id: pulsarr_group_dto.pulsarr_group_id,
        rating_system_id: pulsarr_group_dto.rating_system_id,
        name: pulsarr_group_dto.name.clone(),
        privacy_type: pulsarr_group_dto.privacy_type.clone(),
    }
}
use crate::api::dtos::group_member_dto::GroupMemberDTO;
use crate::api::dtos::rating_system_dto;
use crate::api::dtos::rating_system_dto::RatingSystemDTO;
use crate::data::models::pulsarr_group::PulsarrGroup;
use crate::data::models::rating_system::RatingSystem;
use crate::data::models::rating_system_parameter::RatingSystemParameter;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GroupDTO {
    pub pulsarr_group_id: i32,
    pub rating_system_id: i32,
    pub name: String,
    pub privacy_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_system: Option<RatingSystemDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<NaiveDateTime>,
    pub members: Vec<GroupMemberDTO>,
}

pub fn to_dto(
    pulsarr_group: &PulsarrGroup,
    rating_system: Option<&RatingSystem>,
    parameters: Option<Vec<RatingSystemParameter>>,
) -> GroupDTO {
    GroupDTO {
        pulsarr_group_id: pulsarr_group.pulsarr_group_id,
        rating_system_id: pulsarr_group.rating_system_id,
        name: pulsarr_group.name.clone(),
        privacy_type: pulsarr_group.privacy_type.clone(),
        rating_system: match rating_system {
            Some(rs) => Some(rating_system_dto::to_dto(rs, parameters)),
            None => None,
        },
        created_by_user_id: pulsarr_group.created_by_user_id,
        creation_date: Some(pulsarr_group.creation_date),
        members: Vec::new(),
    }
}

pub fn to_model(pulsarr_group_dto: &GroupDTO) -> PulsarrGroup {
    PulsarrGroup {
        pulsarr_group_id: pulsarr_group_dto.pulsarr_group_id,
        rating_system_id: pulsarr_group_dto.rating_system_id,
        name: pulsarr_group_dto.name.clone(),
        privacy_type: pulsarr_group_dto.privacy_type.clone(),
        created_by_user_id: pulsarr_group_dto.created_by_user_id,
        creation_date: pulsarr_group_dto
            .creation_date
            .unwrap_or_else(|| chrono::Local::now().naive_local()),
    }
}

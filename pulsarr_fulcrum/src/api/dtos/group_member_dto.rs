use crate::api::dtos::user_dto::UserDTO;
use crate::api::dtos::{user_dto};
use crate::data::models::pulsarr_user::PulsarrUser;
use crate::data::models::user_group::UserGroup;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GroupMemberDTO {
    pub pulsarr_group_id: i32,
    pub user: UserDTO,
    pub group_role: String,
    pub join_date: NaiveDateTime,
}

pub fn to_dto(pulsarr_group_id: i32, user_group: &UserGroup, user: &PulsarrUser) -> GroupMemberDTO {
    GroupMemberDTO{
        pulsarr_group_id,
        user: user_dto::to_dto(user),
        group_role: user_group.group_role.to_owned(),
        join_date: user_group.join_date,
    }
}
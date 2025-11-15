use rust_decimal::Decimal;
use crate::api::dtos::{group_dto, rating_system_dto, rating_system_parameter_dto};
use crate::api::dtos::group_dto::GroupDTO;
use crate::api::dtos::rating_system_dto::RatingSystemDTO;
use crate::api::dtos::rating_system_parameter_dto::RatingSystemParameterDTO;
use crate::data::models::pulsarr_group::PulsarrGroup;
use crate::data::models::rating::Rating;
use crate::data::models::rating_detail::RatingDetail;
use crate::data::models::rating_system::RatingSystem;
use crate::data::models::rating_system_parameter::RatingSystemParameter;

pub(crate) struct RatingDTO {
    pub rating_id: i32,
    pub pulsarr_user_id: i32,
    pub pulsarr_group_id: i32,
    pub rating_system_id: i32,
    pub comments: String,
    pub rating_value: Decimal,

    pub rating_details: Vec<RatingDetailDTO>,
    pub group: Option<GroupDTO>,
    pub rating_system: Option<RatingSystemDTO>,
}

pub(crate) struct RatingDetailDTO {
    pub rating_detail_id: i32,
    pub rating_id: i32,
    pub rating_system_parameter_id: i32,
    pub rating_value: Decimal,
    pub rating_system_parameter: Option<RatingSystemParameterDTO>,
}

pub fn to_rating_dto(rating: Rating, details: Vec<RatingDetail>, rating_system: Option<&RatingSystem>, group: Option<&PulsarrGroup>, rating_system_parameters: Vec<RatingSystemParameter>) -> RatingDTO {
    RatingDTO {
        rating_id: rating.rating_id,
        pulsarr_user_id: rating.pulsarr_user_id,
        pulsarr_group_id: rating.pulsarr_group_id,
        rating_system_id: rating.rating_system_id,
        comments: rating.comments,
        rating_value: rating.rating_value,
        rating_details: details.iter().map(
            |d| to_rating_detail_dto(d, rating_system_parameters.iter().find(
                |p| d.rating_system_parameter_id == p.rating_system_parameter_id))
        ).collect(),
        group: match group {
            Some(g) => Some(group_dto::to_dto(g, rating_system, None)),
            None => None
        },
        rating_system: match rating_system {
            Some(rs) => Some(rating_system_dto::to_dto(rs, None)),
            None => None
        },
    }
}

pub fn to_rating_detail_dto(detail: &RatingDetail, parameter: Option<&RatingSystemParameter>) -> RatingDetailDTO {
    RatingDetailDTO {
        rating_detail_id: detail.rating_detail_id,
        rating_id: detail.rating_id,
        rating_system_parameter_id: detail.rating_system_parameter_id,
        rating_value: detail.rating_value,
        rating_system_parameter: match parameter {
            Some(p) => Some(rating_system_parameter_dto::to_dto(p)),
            None => None
        },
    }
}


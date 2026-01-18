use rust_decimal::Decimal;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::data::models::rating_system_parameter::RatingSystemParameter;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RatingSystemParameterDTO {
    pub rating_system_parameter_id: i32,
    pub rating_system_id: i32,
    pub parameter_rating_max: Decimal,
    pub name: String,
    pub weight: Decimal,
}

pub fn to_dto(rating_system_parameter: &RatingSystemParameter) -> RatingSystemParameterDTO {
    RatingSystemParameterDTO {
        rating_system_parameter_id: rating_system_parameter.rating_system_parameter_id,
        rating_system_id: rating_system_parameter.rating_system_id,
        parameter_rating_max: rating_system_parameter.parameter_rating_max,
        name: rating_system_parameter.name.clone(),
        weight: rating_system_parameter.weight,
    }
}

pub fn to_model(rating_system_parameter_dto: &RatingSystemParameterDTO) -> RatingSystemParameter {
    RatingSystemParameter {
        rating_system_parameter_id: rating_system_parameter_dto.rating_system_parameter_id,
        rating_system_id: rating_system_parameter_dto.rating_system_id,
        parameter_rating_max: rating_system_parameter_dto.parameter_rating_max,
        name: rating_system_parameter_dto.name.clone(),
        weight: rating_system_parameter_dto.weight,
    }
}


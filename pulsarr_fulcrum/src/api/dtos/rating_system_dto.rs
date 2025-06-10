use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use rust_decimal::Decimal;
use crate::api::dtos::{rating_system_parameter_dto, rating_system_parameter_dto::RatingSystemParameterDTO};
use crate::data::models::rating_system::RatingSystem;
use crate::data::models::rating_system_parameter::RatingSystemParameter;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RatingSystemDTO {
    pub rating_system_id: i32,
    pub master_rating_type: String,
    pub rating_max: Decimal,
    pub name: String,
    pub parameters: Vec<RatingSystemParameterDTO>,
}

pub fn to_dto(rating_system: &RatingSystem, parameters: Option<Vec<RatingSystemParameter>>) -> RatingSystemDTO {
    RatingSystemDTO {
        rating_system_id: rating_system.rating_system_id,
        master_rating_type: rating_system.master_rating_type.clone(),
        rating_max: rating_system.rating_max,
        name: rating_system.name.clone(),
        parameters: match parameters {
            Some(rsps) => 
                rsps.iter().map(|rsp| rating_system_parameter_dto::to_dto(rsp))
                    .collect::<Vec<RatingSystemParameterDTO>>(),
            None => vec![]
        }
    }
}

pub fn to_model(rating_system_dto: &RatingSystemDTO) -> RatingSystem {
    RatingSystem {
        rating_system_id: rating_system_dto.rating_system_id,
        master_rating_type: rating_system_dto.master_rating_type.clone(),
        rating_max: rating_system_dto.rating_max,
        name: rating_system_dto.name.clone(),
    }
}
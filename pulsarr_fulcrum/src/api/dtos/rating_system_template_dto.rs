use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use rust_decimal::Decimal;
use crate::data::models::rating_system_template::RatingSystemTemplate;
use crate::data::models::rating_system_template_parameter::RatingSystemTemplateParameter;

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct RatingSystemTemplateParameterDTO {
    pub template_parameter_id: i32,
    pub template_id: i32,
    pub name: String,
    pub parameter_rating_max: Decimal,
    pub weight: Decimal,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RatingSystemTemplateDTO {
    pub template_id: i32,
    pub name: String,
    pub master_rating_type: String,
    pub rating_max: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RatingSystemTemplateParameterDTO>>,
}

pub fn to_dto(template: &RatingSystemTemplate, parameters: Option<Vec<RatingSystemTemplateParameter>>) -> RatingSystemTemplateDTO {
    RatingSystemTemplateDTO {
        template_id: template.template_id,
        name: template.name.clone(),
        master_rating_type: template.master_rating_type.clone(),
        rating_max: template.rating_max,
        parameters: parameters.map(|params| {
            params.iter().map(|p| to_parameter_dto(p)).collect()
        }),
    }
}

pub fn to_model(dto: &RatingSystemTemplateDTO) -> RatingSystemTemplate {
    RatingSystemTemplate {
        template_id: dto.template_id,
        name: dto.name.clone(),
        master_rating_type: dto.master_rating_type.clone(),
        rating_max: dto.rating_max,
    }
}

pub fn to_parameter_dto(param: &RatingSystemTemplateParameter) -> RatingSystemTemplateParameterDTO {
    RatingSystemTemplateParameterDTO {
        template_parameter_id: param.template_parameter_id,
        template_id: param.template_id,
        name: param.name.clone(),
        parameter_rating_max: param.parameter_rating_max,
        weight: param.weight,
    }
}

pub fn to_parameter_model(dto: &RatingSystemTemplateParameterDTO) -> RatingSystemTemplateParameter {
    RatingSystemTemplateParameter {
        template_parameter_id: dto.template_parameter_id,
        template_id: dto.template_id,
        name: dto.name.clone(),
        parameter_rating_max: dto.parameter_rating_max,
        weight: dto.weight,
    }
}

use crate::data::data_wrangler;
use crate::data::models::rating_system_template::RatingSystemTemplate;
use crate::data::models::rating_system_template_parameter::{self, RatingSystemTemplateParameter};
use crate::{PostgresState, PulsarrResult};
use rocket::serde::json::Json;
use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use crate::api::dtos::rating_system_template_dto::{self, RatingSystemTemplateDTO};
use crate::error::PulsarrError;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        get_all_templates, get_template, add_template, update_template, delete_template]
}

/// # Get all rating system templates (with parameters hydrated)
#[openapi(tag = "Rating System Template")]
#[get("/")]
async fn get_all_templates(state: &State<PostgresState>) -> PulsarrResult<Vec<RatingSystemTemplateDTO>> {
    let templates = match data_wrangler::get_all::<RatingSystemTemplate>(&state.pool, None, None).await {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let mut result = Vec::new();
    for template in &templates {
        let params = match rating_system_template_parameter::get_by_template_id(template.template_id)
            .fetch_all(&state.pool)
            .await
        {
            Ok(p) => p,
            Err(e) => return Err(PulsarrError::validation_error(e)),
        };
        result.push(rating_system_template_dto::to_dto(template, Some(params)));
    }

    Ok(Json(result))
}

/// # Get a rating system template by ID (with parameters)
#[openapi(tag = "Rating System Template")]
#[get("/<id>")]
async fn get_template(state: &State<PostgresState>, id: i32) -> PulsarrResult<RatingSystemTemplateDTO> {
    let template = match data_wrangler::get_by_id::<RatingSystemTemplate>(id, &state.pool).await {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let params = match rating_system_template_parameter::get_by_template_id(template.template_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(p) => p,
        Err(e) => return Err(PulsarrError::validation_error(e)),
    };

    Ok(Json(rating_system_template_dto::to_dto(&template, Some(params))))
}

/// # Create a new rating system template with parameters
#[openapi(tag = "Rating System Template")]
#[post("/add", format = "application/json", data = "<request_dto>")]
async fn add_template(state: &State<PostgresState>, request_dto: Json<RatingSystemTemplateDTO>) -> PulsarrResult<RatingSystemTemplateDTO> {
    let template = match data_wrangler::add(rating_system_template_dto::to_model(&request_dto), &state.pool).await {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let parameter_models: Vec<RatingSystemTemplateParameter> = request_dto.parameters
        .as_ref()
        .map(|params| {
            params.iter().map(|p| {
                let mut dto = p.clone();
                dto.template_id = template.template_id;
                rating_system_template_dto::to_parameter_model(&dto)
            }).collect()
        })
        .unwrap_or_default();

    let mut parameters = Vec::new();
    for parameter in parameter_models {
        match data_wrangler::add(parameter, &state.pool).await {
            Ok(param) => parameters.push(param),
            Err(e) => return Err(e),
        }
    }

    Ok(Json(rating_system_template_dto::to_dto(&template, Some(parameters))))
}

/// # Update a rating system template
#[openapi(tag = "Rating System Template")]
#[post("/update", format = "application/json", data = "<template>")]
async fn update_template(state: &State<PostgresState>, template: Json<RatingSystemTemplate>) -> PulsarrResult<RatingSystemTemplate> {
    match data_wrangler::update(template.into_inner(), &state.pool).await {
        Ok(t) => Ok(Json(t)),
        Err(e) => Err(e),
    }
}

/// # Delete a rating system template
#[openapi(tag = "Rating System Template")]
#[delete("/delete/<id>")]
async fn delete_template(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool> {
    match data_wrangler::delete::<RatingSystemTemplate>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e),
    }
}

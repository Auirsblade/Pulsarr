use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use crate::{PostgresState, PulsarrResult};
use crate::data::data_wrangler;
use crate::data::models::{rating_system_parameter::RatingSystemParameter, rating_system::{RatingSystem, RATING_TYPE}};

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: 
        add_rating_system, update_rating_system, delete_rating_system, get_rating_system, get_all_rating_systems, get_rating_types, 
        add_rating_system_parameter, update_rating_system_parameter, delete_rating_system_parameter, get_rating_system_parameter, get_all_rating_system_parameters ]
}

/// # Add rating system
#[openapi(tag = "Rating System")]
#[post("/add", format = "application/json", data = "<rating_system>")]
async fn add_rating_system(state: &State<PostgresState>, rating_system: Json<RatingSystem>) -> PulsarrResult<RatingSystem> {
    match data_wrangler::add(rating_system.into_inner(), &state.pool).await{
        Ok(rating_system) => Ok(Json(rating_system)),
        Err(e) => Err(e)
    }
}

/// # Update rating system
#[openapi(tag = "Rating System")]
#[post("/update", format = "application/json", data = "<rating_system>")]
async fn update_rating_system(state: &State<PostgresState>, rating_system: Json<RatingSystem>) -> PulsarrResult<RatingSystem> {
    match data_wrangler::update(rating_system.into_inner(), &state.pool).await{
        Ok(rating_system) => Ok(Json(rating_system)),
        Err(e) => Err(e)
    }
}

/// # Delete rating system
#[openapi(tag = "Rating System")]
#[delete("/delete/<id>")]
async fn delete_rating_system(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool> {
    match data_wrangler::delete::<RatingSystem>(id, &state.pool).await{
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(e)
    }
}

/// # Get a rating system by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system(state: &State<PostgresState>, id: i32) -> PulsarrResult<RatingSystem> {
    match data_wrangler::get_by_id::<RatingSystem>(id, &state.pool).await{
        Ok(rating_system) => Ok(Json(rating_system)),
        Err(e) => Err(e)
    }
}

/// # Get all rating systems
#[openapi(tag = "Rating System")]
#[get("/")]
async fn get_all_rating_systems(state: &State<PostgresState>) -> PulsarrResult<Vec<RatingSystem>> {
    match data_wrangler::get_all::<RatingSystem>(&state.pool).await {
        Ok(rating_systems) => Ok(Json(rating_systems)),
        Err(e) => Err(e)
    }
}

/// # Get the master rating types
#[openapi(tag = "Rating System")]
#[get("/ratingTypes")]
async fn get_rating_types() -> PulsarrResult<Vec<String>> {
    let mut rating_types = vec![];

    for typ in RATING_TYPE {
        rating_types.push(typ.to_owned());
    }

    Ok(Json(rating_types))
}

/// # Add rating system parameter
#[openapi(tag = "Rating System")]
#[post("/parameter/add", format = "application/json", data = "<rating_system_parameter>")]
async fn add_rating_system_parameter(state: &State<PostgresState>, rating_system_parameter: Json<RatingSystemParameter>) -> PulsarrResult<RatingSystemParameter> {
    match data_wrangler::add(rating_system_parameter.into_inner(), &state.pool).await {
        Ok(parameter) => Ok(Json(parameter)),
        Err(e) => Err(e)
    }
}

/// # Update rating system parameter
#[openapi(tag = "Rating System")]
#[post("/parameter/update", format = "application/json", data = "<rating_system_parameter>")]
async fn update_rating_system_parameter(state: &State<PostgresState>, rating_system_parameter: Json<RatingSystemParameter>) -> PulsarrResult<RatingSystemParameter> {
    match data_wrangler::update(rating_system_parameter.into_inner(), &state.pool).await {
        Ok(parameter) => Ok(Json(parameter)),
        Err(e) => Err(e)
    }
}

/// # Delete rating system parameter
#[openapi(tag = "Rating System")]
#[delete("/parameter/delete/<id>")]
async fn delete_rating_system_parameter(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool> {
    match data_wrangler::delete::<RatingSystemParameter>(id, &state.pool).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(e)
    }
}

/// # Get a rating system parameter by id
#[openapi(tag = "Rating System")]
#[get("/parameter/<id>")]
async fn get_rating_system_parameter(state: &State<PostgresState>, id: i32) -> PulsarrResult<RatingSystemParameter> {
    match data_wrangler::get_by_id::<RatingSystemParameter>(id, &state.pool).await {
        Ok(parameter) => Ok(Json(parameter)),
        Err(e) => Err(e)
    }
}

/// # Get all rating system parameters
#[openapi(tag = "Rating System")]
#[get("/parameter")]
async fn get_all_rating_system_parameters(state: &State<PostgresState>) -> PulsarrResult<Vec<RatingSystemParameter>> {
    match data_wrangler::get_all::<RatingSystemParameter>(&state.pool).await {
        Ok(parameters) => Ok(Json(parameters)),
        Err(e) => Err(e)       
    }
}
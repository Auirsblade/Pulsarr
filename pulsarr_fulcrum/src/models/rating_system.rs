use rocket::get;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::{openapi3::OpenApi, schemars::schema_for},
    openapi, 
    openapi_get_routes_spec,
    settings::OpenApiSettings,
    JsonSchema
};
use serde::Serialize;
use crate::schemars;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system]
}

#[derive(Serialize, JsonSchema)]
struct RatingSystem {
    rating_system_id: i32,
    master_rating_type: MasterRatingType,
    adaptive_type_max: i32,
}

#[derive(Serialize, JsonSchema)]
enum MasterRatingType {
    Absolute,
    Cumulative,
    Average,
}

/// # Get a rating system by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system(id: i32) -> crate::Result<RatingSystem> {
    Ok(Json(RatingSystem {
        rating_system_id: id,
        master_rating_type: MasterRatingType::Absolute,
        adaptive_type_max: 10,
    }))
}

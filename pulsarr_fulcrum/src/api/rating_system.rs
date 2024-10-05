use rocket::{get, State};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use crate::PostgresState;
use crate::data::models::{ rating_system_parameter::RatingSystemParameter, rating_system::{ RatingSystem, RATING_TYPE } };

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system, get_rating_types, get_rating_system_parameter]
}

/// # Get a rating system by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystem> {
    let rating_system = sqlx::query_as::<_, RatingSystem>(
        "select * from rating_system where rating_system_id = $1",
    )
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(Json(rating_system))
}

/// # Get the master rating types
#[openapi(tag = "Rating System")]
#[get("/ratingTypes")]
async fn get_rating_types() -> crate::PulsarrResult<Vec<String>> {
    let mut rating_types = vec![];

    for typ in RATING_TYPE {
        rating_types.push(typ.to_owned());
    }

    Ok(Json(rating_types))
}

/// # Get a rating system parameter by id
#[openapi(tag = "Rating System")]
#[get("/parameter/<id>")]
async fn get_rating_system_parameter(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystemParameter> {
    let rating_system = sqlx::query_as::<_, RatingSystemParameter>(
        "select * from rating_system_parameter where rating_system_parameter_id = $1",
    )
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(Json(rating_system))
}
use rocket::{get, State};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::PostgresState;
use crate::data::models::{ rating::Rating, rating_detail::RatingDetail };

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating, get_rating_detail]
}

/// # Get a rating by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<Rating> {
    let group = sqlx::query_as::<_, Rating>("select * from rating where rating_id = $1")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(Json(group))
}

/// # Get a rating detail by id
#[openapi(tag = "Rating")]
#[get("/rating_detail/<id>")]
async fn get_rating_detail(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingDetail> {
    let group = sqlx::query_as::<_, RatingDetail>(
        "select * from rating_detail where rating_detail_id = $1",
    )
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(Json(group))
}
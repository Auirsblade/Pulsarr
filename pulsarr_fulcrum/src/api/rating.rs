use crate::data::data_wrangler;
use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::{PostgresState, PulsarrResult};
use crate::data::models::{rating::Rating, rating_detail::RatingDetail};

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![ settings: 
        add_rating, update_rating, delete_rating, get_rating, get_all_ratings, 
        add_rating_detail, update_rating_detail, delete_rating_detail, get_rating_detail, get_all_rating_details ]
}


/// # Add rating
#[openapi(tag = "Rating")]
#[post("/add", format = "application/json", data = "<rating>")]
async fn add_rating(state: &State<PostgresState>, rating: Json<Rating>) -> PulsarrResult<Rating>{
    data_wrangler::add(rating.into_inner(), &state.pool).await
}

/// # Update rating
#[openapi(tag = "Rating")]
#[post("/update", format = "application/json", data = "<rating>")]
async fn update_rating(state: &State<PostgresState>, rating: Json<Rating>) -> PulsarrResult<Rating>{
    data_wrangler::update(rating.into_inner(), &state.pool).await
}

/// # Delete rating
#[openapi(tag = "Rating")]
#[delete("/delete/<id>")]
async fn delete_rating(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool>{
    data_wrangler::delete::<Rating>(id, &state.pool).await
}

/// # Get a rating by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating(state: &State<PostgresState>, id: i32) -> PulsarrResult<Rating> {
    data_wrangler::get_by_id::<Rating>(id, &state.pool).await
}

/// # Get all ratings
#[openapi(tag = "Rating")]
#[get("/")]
async fn get_all_ratings(state: &State<PostgresState>) -> PulsarrResult<Vec<Rating>> {
    data_wrangler::get_all::<Rating>(&state.pool).await
}

/// # Add rating detail
#[openapi(tag = "Rating")]
#[post("/rating_detail/add", format = "application/json", data = "<rating_detail>")]
async fn add_rating_detail(state: &State<PostgresState>, rating_detail: Json<RatingDetail>) -> PulsarrResult<RatingDetail>{
    data_wrangler::add(rating_detail.into_inner(), &state.pool).await
}

/// # Update rating detail
#[openapi(tag = "Rating")]
#[post("/rating_detail/update", format = "application/json", data = "<rating_detail>")]
async fn update_rating_detail(state: &State<PostgresState>, rating_detail: Json<RatingDetail>) -> PulsarrResult<RatingDetail>{
    data_wrangler::update(rating_detail.into_inner(), &state.pool).await
}

/// # Delete rating detail 
#[openapi(tag = "Rating")]
#[delete("/rating_detail/delete/<id>")]
async fn delete_rating_detail(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool>{
    data_wrangler::delete::<RatingDetail>(id, &state.pool).await
}

/// # Get a rating detail by id
#[openapi(tag = "Rating")]
#[get("/rating_detail/<id>")]
async fn get_rating_detail(state: &State<PostgresState>, id: i32) -> PulsarrResult<RatingDetail> {
    data_wrangler::get_by_id::<RatingDetail>(id, &state.pool).await
}

/// # Get all rating details
#[openapi(tag = "Rating")]
#[get("/rating_detail")]
async fn get_all_rating_details(state: &State<PostgresState>) -> PulsarrResult<Vec<RatingDetail>> {
    data_wrangler::get_all::<RatingDetail>(&state.pool).await
}
use crate::data::data_wrangler;
use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use sqlx::query_as;
use crate::{PostgresState, PulsarrResult};
use crate::data::models::{rating::Rating, rating_detail::RatingDetail};
use crate::api::dtos::rating_dto::{CreateRatingDTO, create_rating_to_model};

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![ settings:
        add_rating, update_rating, delete_rating, get_rating, get_all_ratings, get_ratings_by_group,
        add_rating_detail, update_rating_detail, delete_rating_detail, get_rating_detail, get_all_rating_details ]
}


/// # Add rating
#[openapi(tag = "Rating")]
#[post("/add", format = "application/json", data = "<rating>")]
async fn add_rating(state: &State<PostgresState>, rating: Json<CreateRatingDTO>) -> PulsarrResult<Rating>{
    let rating_model = create_rating_to_model(rating.into_inner());
    match data_wrangler::add(rating_model, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Update rating
#[openapi(tag = "Rating")]
#[post("/update", format = "application/json", data = "<rating>")]
async fn update_rating(state: &State<PostgresState>, rating: Json<Rating>) -> PulsarrResult<Rating>{
    match data_wrangler::update(rating.into_inner(), &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Delete rating
#[openapi(tag = "Rating")]
#[delete("/delete/<id>")]
async fn delete_rating(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool>{
    match data_wrangler::delete::<Rating>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get a rating by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating(state: &State<PostgresState>, id: i32) -> PulsarrResult<Rating> {
    match data_wrangler::get_by_id::<Rating>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get all ratings
#[openapi(tag = "Rating")]
#[get("/")]
async fn get_all_ratings(state: &State<PostgresState>) -> PulsarrResult<Vec<Rating>> {
    match data_wrangler::get_all::<Rating>(&state.pool, None).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get ratings by group IDs
#[openapi(tag = "Rating")]
#[post("/byGroups", format = "application/json", data = "<group_ids>")]
async fn get_ratings_by_group(state: &State<PostgresState>, group_ids: Json<Vec<i32>>) -> PulsarrResult<Vec<Rating>> {
    let ids = group_ids.into_inner();
    if ids.is_empty() {
        return Ok(Json(Vec::new()));
    }

    match query_as::<_, Rating>("SELECT * FROM rating WHERE pulsarr_group_id = ANY($1) ORDER BY rating_date DESC")
        .bind(&ids)
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(_) => Err(rocket::http::Status::InternalServerError)
    }
}

/// # Add rating detail
#[openapi(tag = "Rating")]
#[post("/rating_detail/add", format = "application/json", data = "<rating_detail>")]
async fn add_rating_detail(state: &State<PostgresState>, rating_detail: Json<RatingDetail>) -> PulsarrResult<RatingDetail>{
    match data_wrangler::add(rating_detail.into_inner(), &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Update rating detail
#[openapi(tag = "Rating")]
#[post("/rating_detail/update", format = "application/json", data = "<rating_detail>")]
async fn update_rating_detail(state: &State<PostgresState>, rating_detail: Json<RatingDetail>) -> PulsarrResult<RatingDetail>{
    match data_wrangler::update(rating_detail.into_inner(), &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Delete rating detail 
#[openapi(tag = "Rating")]
#[delete("/rating_detail/delete/<id>")]
async fn delete_rating_detail(state: &State<PostgresState>, id: i32) -> PulsarrResult<bool>{
    match data_wrangler::delete::<RatingDetail>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get a rating detail by id
#[openapi(tag = "Rating")]
#[get("/rating_detail/<id>")]
async fn get_rating_detail(state: &State<PostgresState>, id: i32) -> PulsarrResult<RatingDetail> {
    match data_wrangler::get_by_id::<RatingDetail>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get all rating details
#[openapi(tag = "Rating")]
#[get("/rating_detail")]
async fn get_all_rating_details(state: &State<PostgresState>) -> PulsarrResult<Vec<RatingDetail>> {
    match data_wrangler::get_all::<RatingDetail>(&state.pool, None).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}
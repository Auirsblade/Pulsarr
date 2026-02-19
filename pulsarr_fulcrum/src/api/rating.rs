use crate::data::data_wrangler;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post, State};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use rocket_okapi::settings::OpenApiSettings;
use sqlx::{query_as, FromRow};
use sqlx::types::Decimal;
use crate::{PostgresState, PulsarrResult};
use crate::data::models::{rating::Rating, rating_detail::RatingDetail};
use crate::api::dtos::rating_dto::{CreateRatingDTO, create_rating_to_model};
use crate::api::guards::api_key::ApiKey;
use crate::error::PulsarrError;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PaginatedGroupRatingsRequest {
    pub group_ids: Vec<i32>,
    pub take_size: Option<i32>,
    pub offset: Option<i32>,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![ settings:
        add_rating, update_rating, delete_rating, get_rating, get_all_ratings, get_ratings_by_group,
        get_ratings_by_user, get_user_rating_stats,
        add_rating_detail, update_rating_detail, delete_rating_detail, get_rating_detail, get_all_rating_details,
        get_rating_details_by_rating ]
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
    match data_wrangler::get_all::<Rating>(&state.pool, None, None).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get ratings by group IDs
#[openapi(tag = "Rating")]
#[post("/byGroups", format = "application/json", data = "<request>")]
async fn get_ratings_by_group(state: &State<PostgresState>, request: Json<PaginatedGroupRatingsRequest>) -> PulsarrResult<Vec<Rating>> {
    let req = request.into_inner();
    if req.group_ids.is_empty() {
        return Ok(Json(Vec::new()));
    }

    match query_as::<_, Rating>("SELECT * FROM rating WHERE pulsarr_group_id = ANY($1) ORDER BY rating_date DESC LIMIT $2 OFFSET $3")
        .bind(&req.group_ids)
        .bind(req.take_size)
        .bind(req.offset.unwrap_or(0))
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::validation_error(e))
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
    match data_wrangler::get_all::<RatingDetail>(&state.pool, None, None).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get rating details by rating ID
#[openapi(tag = "Rating")]
#[get("/rating_detail/by_rating/<rating_id>")]
async fn get_rating_details_by_rating(state: &State<PostgresState>, rating_id: i32) -> PulsarrResult<Vec<RatingDetail>> {
    match query_as::<_, RatingDetail>("SELECT * FROM rating_detail WHERE rating_id = $1")
        .bind(rating_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(details) => Ok(Json(details)),
        Err(e) => Err(PulsarrError::validation_error(e))
    }
}

/// # Get ratings by authenticated user
#[openapi(tag = "Rating")]
#[get("/byUser?<take_size>&<offset>")]
async fn get_ratings_by_user(
    state: &State<PostgresState>,
    api_user: ApiKey,
    take_size: Option<i32>,
    offset: Option<i32>,
) -> PulsarrResult<Vec<Rating>> {
    let ApiKey(user_id) = api_user;
    let limit = take_size.unwrap_or(20);
    let off = offset.unwrap_or(0);

    match query_as::<_, Rating>("SELECT * FROM rating WHERE pulsarr_user_id = $1 ORDER BY rating_date DESC LIMIT $2 OFFSET $3")
        .bind(user_id)
        .bind(limit)
        .bind(off)
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::validation_error(e))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct ArtistCount {
    artist_name: String,
    count: i64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct UserRatingStats {
    total_ratings: i64,
    average_rating: Option<Decimal>,
    top_artists: Vec<ArtistCount>,
}

#[derive(FromRow)]
struct CountRow {
    count: i64,
}

#[derive(FromRow)]
struct AvgRow {
    avg: Option<Decimal>,
}

#[derive(FromRow)]
struct ArtistCountRow {
    artist_name: String,
    count: i64,
}

/// # Get rating stats for authenticated user
#[openapi(tag = "Rating")]
#[get("/stats")]
async fn get_user_rating_stats(
    state: &State<PostgresState>,
    api_user: ApiKey,
) -> PulsarrResult<UserRatingStats> {
    let ApiKey(user_id) = api_user;

    let count_row = query_as::<_, CountRow>("SELECT COUNT(*) as count FROM rating WHERE pulsarr_user_id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    let avg_row = query_as::<_, AvgRow>("SELECT AVG(rating_value) as avg FROM rating WHERE pulsarr_user_id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    let top_artists = query_as::<_, ArtistCountRow>(
        "SELECT artist_name, COUNT(*) as count FROM rating WHERE pulsarr_user_id = $1 GROUP BY artist_name ORDER BY count DESC LIMIT 5"
    )
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    Ok(Json(UserRatingStats {
        total_ratings: count_row.count,
        average_rating: avg_row.avg,
        top_artists: top_artists.into_iter().map(|a| ArtistCount {
            artist_name: a.artist_name,
            count: a.count,
        }).collect(),
    }))
}
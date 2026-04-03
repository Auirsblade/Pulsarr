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

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub struct RatingDetailResponse {
    pub rating_detail_id: i32,
    pub rating_id: i32,
    pub rating_system_parameter_id: i32,
    pub rating_value: Decimal,
    pub parameter_name: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PaginatedGroupRatingsRequest {
    pub group_ids: Vec<i32>,
    pub take_size: Option<i32>,
    pub offset: Option<i32>,
    pub current_only: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GroupAlbumKey {
    pub group_id: i32,
    pub musicbrainz_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GroupAveragesRequest {
    pub items: Vec<GroupAlbumKey>,
}

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub struct GroupAverageResponse {
    pub pulsarr_group_id: i32,
    pub musicbrainz_id: String,
    pub average_score: Decimal,
    pub rating_count: i64,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![ settings:
        add_rating, update_rating, delete_rating, get_rating, get_all_ratings, get_ratings_by_group,
        get_ratings_by_user, get_user_rating_stats, get_existing_rating, get_group_averages, get_ratings_by_album,
        add_rating_detail, update_rating_detail, delete_rating_detail, get_rating_detail, get_all_rating_details,
        get_rating_details_by_rating, delete_rating_details_by_rating ]
}


/// # Add rating
#[openapi(tag = "Rating")]
#[post("/add", format = "application/json", data = "<rating>")]
async fn add_rating(state: &State<PostgresState>, rating: Json<CreateRatingDTO>, api_user: ApiKey) -> PulsarrResult<Rating>{
    let ApiKey(user_id) = api_user;
    let dto = rating.into_inner();

    if dto.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot create ratings for another user"));
    }

    // Check for existing rating with same user/group/media combo
    let existing = query_as::<_, Rating>(
        "SELECT * FROM rating WHERE pulsarr_user_id = $1 AND pulsarr_group_id = $2 AND musicbrainz_id = $3"
    )
        .bind(user_id)
        .bind(dto.pulsarr_group_id)
        .bind(&dto.musicbrainz_id)
        .fetch_optional(&state.pool)
        .await;

    if let Ok(Some(_)) = existing {
        return Err(PulsarrError::validation_error("A rating already exists for this media in this group. Use update instead."));
    }

    let rating_model = create_rating_to_model(dto);
    match data_wrangler::add(rating_model, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Update rating
#[openapi(tag = "Rating")]
#[post("/update", format = "application/json", data = "<rating>")]
async fn update_rating(state: &State<PostgresState>, rating: Json<Rating>, api_user: ApiKey) -> PulsarrResult<Rating>{
    let ApiKey(user_id) = api_user;

    let original = data_wrangler::get_by_id::<Rating>(rating.rating_id, &state.pool).await?;
    if original.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot update another user's rating"));
    }

    match data_wrangler::update(rating.into_inner(), &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Delete rating
#[openapi(tag = "Rating")]
#[delete("/delete/<id>")]
async fn delete_rating(state: &State<PostgresState>, id: i32, api_user: ApiKey) -> PulsarrResult<bool>{
    let ApiKey(user_id) = api_user;

    let original = data_wrangler::get_by_id::<Rating>(id, &state.pool).await?;
    if original.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot delete another user's rating"));
    }

    match data_wrangler::delete::<Rating>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get a rating by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating(state: &State<PostgresState>, id: i32, _api_user: ApiKey) -> PulsarrResult<Rating> {
    match data_wrangler::get_by_id::<Rating>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get all ratings
#[openapi(tag = "Rating")]
#[get("/")]
async fn get_all_ratings(state: &State<PostgresState>, _api_user: ApiKey) -> PulsarrResult<Vec<Rating>> {
    match data_wrangler::get_all::<Rating>(&state.pool, None, None).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get ratings by group IDs
#[openapi(tag = "Rating")]
#[post("/byGroups", format = "application/json", data = "<request>")]
async fn get_ratings_by_group(state: &State<PostgresState>, request: Json<PaginatedGroupRatingsRequest>, _api_user: ApiKey) -> PulsarrResult<Vec<Rating>> {
    let req = request.into_inner();
    if req.group_ids.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let limit = req.take_size.unwrap_or(20).min(100);

    let result = if req.current_only.unwrap_or(false) {
        query_as::<_, Rating>(
            "SELECT r.*, u.name AS user_name FROM rating r \
             JOIN pulsarr_group g ON g.pulsarr_group_id = r.pulsarr_group_id \
             LEFT JOIN pulsarr_user u ON u.pulsarr_user_id = r.pulsarr_user_id \
             WHERE r.pulsarr_group_id = ANY($1) AND r.rating_system_id = g.rating_system_id \
             ORDER BY r.rating_date DESC LIMIT $2 OFFSET $3"
        )
            .bind(&req.group_ids)
            .bind(limit)
            .bind(req.offset.unwrap_or(0))
            .fetch_all(&state.pool)
            .await
    } else {
        query_as::<_, Rating>(
            "SELECT r.*, u.name AS user_name FROM rating r \
             LEFT JOIN pulsarr_user u ON u.pulsarr_user_id = r.pulsarr_user_id \
             WHERE r.pulsarr_group_id = ANY($1) ORDER BY r.rating_date DESC LIMIT $2 OFFSET $3"
        )
            .bind(&req.group_ids)
            .bind(limit)
            .bind(req.offset.unwrap_or(0))
            .fetch_all(&state.pool)
            .await
    };

    match result {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch ratings by group", e))
    }
}

/// # Add rating detail
#[openapi(tag = "Rating")]
#[post("/rating_detail/add", format = "application/json", data = "<rating_detail>")]
async fn add_rating_detail(state: &State<PostgresState>, rating_detail: Json<RatingDetail>, api_user: ApiKey) -> PulsarrResult<RatingDetail>{
    let ApiKey(user_id) = api_user;
    let detail = rating_detail.into_inner();

    let parent = data_wrangler::get_by_id::<Rating>(detail.rating_id, &state.pool).await?;
    if parent.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot add details to another user's rating"));
    }

    match data_wrangler::add(detail, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Update rating detail
#[openapi(tag = "Rating")]
#[post("/rating_detail/update", format = "application/json", data = "<rating_detail>")]
async fn update_rating_detail(state: &State<PostgresState>, rating_detail: Json<RatingDetail>, api_user: ApiKey) -> PulsarrResult<RatingDetail>{
    let ApiKey(user_id) = api_user;
    let detail = rating_detail.into_inner();

    let parent = data_wrangler::get_by_id::<Rating>(detail.rating_id, &state.pool).await?;
    if parent.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot update details on another user's rating"));
    }

    match data_wrangler::update(detail, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Delete rating detail
#[openapi(tag = "Rating")]
#[delete("/rating_detail/delete/<id>")]
async fn delete_rating_detail(state: &State<PostgresState>, id: i32, api_user: ApiKey) -> PulsarrResult<bool>{
    let ApiKey(user_id) = api_user;

    let detail = data_wrangler::get_by_id::<RatingDetail>(id, &state.pool).await?;
    let parent = data_wrangler::get_by_id::<Rating>(detail.rating_id, &state.pool).await?;
    if parent.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot delete details on another user's rating"));
    }

    match data_wrangler::delete::<RatingDetail>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get a rating detail by id
#[openapi(tag = "Rating")]
#[get("/rating_detail/<id>")]
async fn get_rating_detail(state: &State<PostgresState>, id: i32, _api_user: ApiKey) -> PulsarrResult<RatingDetail> {
    match data_wrangler::get_by_id::<RatingDetail>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get all rating details
#[openapi(tag = "Rating")]
#[get("/rating_detail")]
async fn get_all_rating_details(state: &State<PostgresState>, _api_user: ApiKey) -> PulsarrResult<Vec<RatingDetail>> {
    match data_wrangler::get_all::<RatingDetail>(&state.pool, None, None).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get rating details by rating ID
#[openapi(tag = "Rating")]
#[get("/rating_detail/by_rating/<rating_id>")]
async fn get_rating_details_by_rating(state: &State<PostgresState>, rating_id: i32, _api_user: ApiKey) -> PulsarrResult<Vec<RatingDetailResponse>> {
    match query_as::<_, RatingDetailResponse>(
        "SELECT rd.rating_detail_id, rd.rating_id, rd.rating_system_parameter_id, rd.rating_value, \
         rsp.name AS parameter_name \
         FROM rating_detail rd \
         JOIN rating_system_parameter rsp ON rsp.rating_system_parameter_id = rd.rating_system_parameter_id \
         WHERE rd.rating_id = $1"
    )
        .bind(rating_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(details) => Ok(Json(details)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch rating details", e))
    }
}

/// # Delete all rating details for a rating (ownership verified)
#[openapi(tag = "Rating")]
#[delete("/rating_detail/by_rating/<rating_id>")]
async fn delete_rating_details_by_rating(state: &State<PostgresState>, rating_id: i32, api_user: ApiKey) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;

    let original = data_wrangler::get_by_id::<Rating>(rating_id, &state.pool).await?;
    if original.pulsarr_user_id != user_id {
        return Err(PulsarrError::forbidden("Cannot delete details for another user's rating"));
    }

    match sqlx::query("DELETE FROM rating_detail WHERE rating_id = $1")
        .bind(rating_id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::internal_error("Failed to delete rating details", e))
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
    let limit = take_size.unwrap_or(20).min(100);
    let off = offset.unwrap_or(0);

    match query_as::<_, Rating>(
        "SELECT r.*, u.name AS user_name FROM rating r \
         LEFT JOIN pulsarr_user u ON u.pulsarr_user_id = r.pulsarr_user_id \
         WHERE r.pulsarr_user_id = $1 ORDER BY r.rating_date DESC LIMIT $2 OFFSET $3"
    )
        .bind(user_id)
        .bind(limit)
        .bind(off)
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch ratings by user", e))
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
        .map_err(|e| PulsarrError::internal_error("Failed to fetch rating count", e))?;

    let avg_row = query_as::<_, AvgRow>("SELECT AVG(rating_value) as avg FROM rating WHERE pulsarr_user_id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| PulsarrError::internal_error("Failed to fetch rating average", e))?;

    let top_artists = query_as::<_, ArtistCountRow>(
        "SELECT artist_name, COUNT(*) as count FROM rating WHERE pulsarr_user_id = $1 GROUP BY artist_name ORDER BY count DESC LIMIT 5"
    )
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| PulsarrError::internal_error("Failed to fetch top artists", e))?;

    Ok(Json(UserRatingStats {
        total_ratings: count_row.count,
        average_rating: avg_row.avg,
        top_artists: top_artists.into_iter().map(|a| ArtistCount {
            artist_name: a.artist_name,
            count: a.count,
        }).collect(),
    }))
}

/// # Get group averages for a batch of (group_id, musicbrainz_id) pairs
#[openapi(tag = "Rating")]
#[post("/group-averages", format = "application/json", data = "<request>")]
async fn get_group_averages(
    state: &State<PostgresState>,
    request: Json<GroupAveragesRequest>,
    _api_user: ApiKey,
) -> PulsarrResult<Vec<GroupAverageResponse>> {
    let req = request.into_inner();
    if req.items.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let group_ids: Vec<i32> = req.items.iter().map(|i| i.group_id).collect();
    let musicbrainz_ids: Vec<String> = req.items.iter().map(|i| i.musicbrainz_id.clone()).collect();

    match query_as::<_, GroupAverageResponse>(
        "SELECT r.pulsarr_group_id, r.musicbrainz_id, \
         AVG(r.rating_value)::numeric(8,3) AS average_score, \
         COUNT(*)::bigint AS rating_count \
         FROM rating r \
         INNER JOIN unnest($1::int[], $2::text[]) AS keys(gid, mbid) \
         ON r.pulsarr_group_id = keys.gid AND r.musicbrainz_id = keys.mbid \
         GROUP BY r.pulsarr_group_id, r.musicbrainz_id"
    )
        .bind(&group_ids)
        .bind(&musicbrainz_ids)
        .fetch_all(&state.pool)
        .await
    {
        Ok(averages) => Ok(Json(averages)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch group averages", e))
    }
}

/// # Get all ratings for an album in a group
#[openapi(tag = "Rating")]
#[get("/by-album/<group_id>/<musicbrainz_id>")]
async fn get_ratings_by_album(
    state: &State<PostgresState>,
    group_id: i32,
    musicbrainz_id: &str,
    _api_user: ApiKey,
) -> PulsarrResult<Vec<Rating>> {
    match query_as::<_, Rating>(
        "SELECT r.*, u.name AS user_name FROM rating r \
         LEFT JOIN pulsarr_user u ON u.pulsarr_user_id = r.pulsarr_user_id \
         WHERE r.pulsarr_group_id = $1 AND r.musicbrainz_id = $2 \
         ORDER BY r.rating_date DESC"
    )
        .bind(group_id)
        .bind(musicbrainz_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch ratings by album", e))
    }
}

/// # Get existing ratings for a specific media item in a group (for re-rating reference)
#[openapi(tag = "Rating")]
#[get("/existing/<group_id>/<musicbrainz_id>")]
async fn get_existing_rating(
    state: &State<PostgresState>,
    group_id: i32,
    musicbrainz_id: &str,
    api_user: ApiKey,
) -> PulsarrResult<Vec<Rating>> {
    let ApiKey(user_id) = api_user;

    match query_as::<_, Rating>(
        "SELECT r.*, u.name AS user_name FROM rating r \
         LEFT JOIN pulsarr_user u ON u.pulsarr_user_id = r.pulsarr_user_id \
         WHERE r.pulsarr_user_id = $1 AND r.pulsarr_group_id = $2 AND r.musicbrainz_id = $3 ORDER BY r.rating_date DESC"
    )
        .bind(user_id)
        .bind(group_id)
        .bind(musicbrainz_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch existing rating", e))
    }
}

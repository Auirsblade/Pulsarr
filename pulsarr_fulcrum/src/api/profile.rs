use crate::api::dtos::get_request::GetRequest;
use crate::api::dtos::group_dto::{self, GroupDTO};
use crate::api::dtos::user_dto::{self, UserDTO};
use crate::api::guards::api_key::ApiKey;
use crate::constants::PERSONAL_PRIVACY_TYPE;
use crate::data::data_wrangler;
use crate::data::models::pulsarr_group::{self, PulsarrGroup};
use crate::data::models::pulsarr_user::PulsarrUser;
use crate::data::models::rating::Rating;
use crate::data::models::rating_system::RatingSystem;
use crate::data::models::rating_system_parameter;
use crate::data::models::user_group;
use crate::error::PulsarrError;
use crate::{PostgresState, PulsarrResult};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use sqlx::query_as;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_user_profile, get_profile_ratings]
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserProfileResponse {
    pub user: UserDTO,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_group: Option<GroupDTO>,
    pub visible_group_ids: Vec<i32>,
    pub is_own_profile: bool,
    pub is_private: bool,
}

/// # Get a user's profile with visibility information
#[openapi(tag = "Profile")]
#[get("/<user_id>")]
async fn get_user_profile(
    state: &State<PostgresState>,
    user_id: i32,
    api_user: ApiKey,
) -> PulsarrResult<UserProfileResponse> {
    let ApiKey(caller_id) = api_user;
    let target_user = data_wrangler::get_by_id::<PulsarrUser>(user_id, &state.pool).await?;
    let is_own = caller_id == user_id;

    // If profile is private and not own, return minimal info
    if !is_own && target_user.profile_visibility == "private" {
        return Ok(Json(UserProfileResponse {
            user: UserDTO {
                pulsarr_user_id: target_user.pulsarr_user_id,
                name: target_user.name.clone(),
                email: String::new(),
                join_date: Some(target_user.join_date),
                profile_visibility: None,
                crate_visibility: None,
            },
            crate_group: None,
            visible_group_ids: vec![],
            is_own_profile: false,
            is_private: true,
        }));
    }

    // Get target user's crate
    let crate_group = pulsarr_group::get_personal_group_by_user::<PulsarrGroup>(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    // Build crate DTO if visible (own profile or crate is public)
    let crate_dto = if let Some(ref pg) = crate_group {
        if is_own || target_user.crate_visibility == "public" {
            let rs = data_wrangler::get_by_id::<RatingSystem>(pg.rating_system_id, &state.pool).await?;
            let params = rating_system_parameter::get_by_rating_system_id(rs.rating_system_id)
                .fetch_all(&state.pool)
                .await
                .map_err(PulsarrError::validation_error)?;
            Some(group_dto::to_dto(pg, Some(&rs), Some(params)))
        } else {
            None
        }
    } else {
        None
    };

    // Compute visible group IDs for ratings
    let mut visible_group_ids: Vec<i32> = Vec::new();

    // Include crate if visible
    if let Some(ref cg) = crate_group {
        if is_own || target_user.crate_visibility == "public" {
            visible_group_ids.push(cg.pulsarr_group_id);
        }
    }

    // Get target user's group memberships
    let target_memberships = user_group::get_by_user_id(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    // Get caller's group memberships (for shared private group check)
    let caller_group_ids: Vec<i32> = if is_own {
        target_memberships.iter().map(|m| m.pulsarr_group_id).collect()
    } else {
        user_group::get_by_user_id(caller_id)
            .fetch_all(&state.pool)
            .await
            .map_err(PulsarrError::validation_error)?
            .iter()
            .map(|m| m.pulsarr_group_id)
            .collect()
    };

    // For each of the target user's groups, check if it's visible to the caller
    for membership in &target_memberships {
        let group_id = membership.pulsarr_group_id;
        // Skip crate (already handled above)
        if crate_group.as_ref().is_some_and(|cg| cg.pulsarr_group_id == group_id) {
            continue;
        }

        let group = data_wrangler::get_by_id::<PulsarrGroup>(group_id, &state.pool).await?;

        if group.privacy_type == PERSONAL_PRIVACY_TYPE {
            continue; // Skip other personal groups (shouldn't happen, but defensive)
        }

        if group.privacy_type == "Public" {
            visible_group_ids.push(group_id);
        } else if caller_group_ids.contains(&group_id) {
            // Caller is also a member of this private group
            visible_group_ids.push(group_id);
        }
    }

    Ok(Json(UserProfileResponse {
        user: if is_own {
            user_dto::to_dto(&target_user)
        } else {
            UserDTO {
                pulsarr_user_id: target_user.pulsarr_user_id,
                name: target_user.name.clone(),
                email: String::new(),
                join_date: Some(target_user.join_date),
                profile_visibility: None,
                crate_visibility: None,
            }
        },
        crate_group: crate_dto,
        visible_group_ids,
        is_own_profile: is_own,
        is_private: false,
    }))
}

/// # Get paginated ratings for a user's profile (filtered by visibility)
#[openapi(tag = "Profile")]
#[post("/<user_id>/ratings", format = "application/json", data = "<request>")]
async fn get_profile_ratings(
    state: &State<PostgresState>,
    user_id: i32,
    request: Json<GetRequest>,
    api_user: ApiKey,
) -> PulsarrResult<Vec<Rating>> {
    let ApiKey(caller_id) = api_user;
    let target_user = data_wrangler::get_by_id::<PulsarrUser>(user_id, &state.pool).await?;
    let is_own = caller_id == user_id;

    // If profile is private and not own, return empty
    if !is_own && target_user.profile_visibility == "private" {
        return Ok(Json(vec![]));
    }

    // Compute visible group IDs (same logic as get_user_profile)
    let mut visible_group_ids: Vec<i32> = Vec::new();

    let crate_group = pulsarr_group::get_personal_group_by_user::<PulsarrGroup>(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    if let Some(ref cg) = crate_group {
        if is_own || target_user.crate_visibility == "public" {
            visible_group_ids.push(cg.pulsarr_group_id);
        }
    }

    let target_memberships = user_group::get_by_user_id(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(PulsarrError::validation_error)?;

    let caller_group_ids: Vec<i32> = if is_own {
        target_memberships.iter().map(|m| m.pulsarr_group_id).collect()
    } else {
        user_group::get_by_user_id(caller_id)
            .fetch_all(&state.pool)
            .await
            .map_err(PulsarrError::validation_error)?
            .iter()
            .map(|m| m.pulsarr_group_id)
            .collect()
    };

    for membership in &target_memberships {
        let group_id = membership.pulsarr_group_id;
        if crate_group.as_ref().is_some_and(|cg| cg.pulsarr_group_id == group_id) {
            continue;
        }

        let group = data_wrangler::get_by_id::<PulsarrGroup>(group_id, &state.pool).await?;
        if group.privacy_type == PERSONAL_PRIVACY_TYPE {
            continue;
        }
        if group.privacy_type == "Public" || caller_group_ids.contains(&group_id) {
            visible_group_ids.push(group_id);
        }
    }

    if visible_group_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    let req = request.into_inner();
    let limit = req.take_size.unwrap_or(20).min(100);
    let off = req.offset.unwrap_or(0);

    match query_as::<_, Rating>(
        "SELECT r.*, u.name AS user_name FROM rating r \
         LEFT JOIN pulsarr_user u ON u.pulsarr_user_id = r.pulsarr_user_id \
         WHERE r.pulsarr_user_id = $1 AND r.pulsarr_group_id = ANY($2) \
         ORDER BY r.rating_date DESC LIMIT $3 OFFSET $4"
    )
        .bind(user_id)
        .bind(&visible_group_ids)
        .bind(limit)
        .bind(off)
        .fetch_all(&state.pool)
        .await
    {
        Ok(ratings) => Ok(Json(ratings)),
        Err(e) => Err(PulsarrError::internal_error("Failed to fetch profile ratings", e)),
    }
}

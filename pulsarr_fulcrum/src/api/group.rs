use crate::api::dtos::get_request::GetRequest;
use crate::api::dtos::group_dto::GroupDTO;
use crate::api::dtos::{
    group_dto, group_member_dto, rating_system_dto, rating_system_parameter_dto,
};
use crate::api::guards::api_key::ApiKey;
use crate::api::guards::authorization;
use crate::constants::{ADMIN_ROLE, MEMBERSHIP_TYPE, MEMBER_ROLE, OWNER_ROLE, PRIVATE_PRIVACY_TYPE, PRIVACY_TYPE};
use crate::data::data_wrangler;
use crate::data::models::pulsarr_group;
use crate::data::models::pulsarr_group::PulsarrGroup;
use crate::data::models::pulsarr_user::PulsarrUser;
use crate::data::models::rating_system::RatingSystem;
use crate::data::models::user_group;
use crate::data::models::rating_system_parameter::RatingSystemParameter;
use crate::data::models::{pulsarr_user, rating_system_parameter};
use crate::error::PulsarrError;
use crate::{PostgresState, PulsarrResult};
use rocket::serde::json::Json;
use rocket::{get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: update_group, delete_group, get_pulsarr_group,
        get_all_groups, get_privacy_types, create_group, get_membership_types, join_group, leave_group,
        search_groups, my_groups, get_public_groups_endpoint, get_group_preview_endpoint,
        kick_member, change_role, transfer_ownership]
}

/// # Get the group privacy types
#[openapi(tag = "Group")]
#[get("/privacyTypes")]
async fn get_privacy_types() -> PulsarrResult<Vec<String>> {
    let mut privacy_types = vec![];

    for typ in PRIVACY_TYPE {
        privacy_types.push(typ.to_owned());
    }

    Ok(Json(privacy_types))
}

/// # Update group (requires Admin+ role)
#[openapi(tag = "Group")]
#[post("/update", format = "application/json", data = "<group>")]
async fn update_group(
    state: &State<PostgresState>,
    group: Json<GroupDTO>,
    api_user: ApiKey,
) -> PulsarrResult<GroupDTO> {
    let ApiKey(user_id) = api_user;
    authorization::require_role(&state.pool, group.pulsarr_group_id, user_id, ADMIN_ROLE).await?;
    match data_wrangler::update(group_dto::to_model(&group), &state.pool).await {
        Ok(r) => Ok(Json(group_dto::to_dto(&r, None, None))),
        Err(e) => Err(e),
    }
}

/// # Delete group (requires Owner role)
#[openapi(tag = "Group")]
#[post("/delete/<id>")]
async fn delete_group(
    state: &State<PostgresState>,
    id: i32,
    api_user: ApiKey,
) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    authorization::require_role(&state.pool, id, user_id, OWNER_ROLE).await?;
    match data_wrangler::delete::<PulsarrGroup>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e),
    }
}

/// # Get a group by id
#[openapi(tag = "Group")]
#[get("/<id>")]
async fn get_pulsarr_group(
    state: &State<PostgresState>,
    id: i32,
    api_user: ApiKey,
) -> PulsarrResult<GroupDTO> {
    let ApiKey(user_id) = api_user;
    let pg = match data_wrangler::get_by_id::<PulsarrGroup>(id, &state.pool).await {
        Ok(pg) => pg,
        Err(e) => return Err(e),
    };

    // If group is private, verify membership
    if pg.privacy_type == PRIVATE_PRIVACY_TYPE {
        if authorization::get_membership(&state.pool, id, user_id).await.is_none() {
            return Err(PulsarrError::forbidden("This group is private"));
        }
    }

    let rs = match data_wrangler::get_by_id::<RatingSystem>(pg.rating_system_id, &state.pool).await
    {
        Ok(rs) => rs,
        Err(e) => return Err(e),
    };

    let parameters = match rating_system_parameter::get_by_rating_system_id(rs.rating_system_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(parameters) => parameters,
        Err(e) => return Err(PulsarrError::validation_error(e)),
    };

    let memberships = match user_group::get_by_group_id(pg.pulsarr_group_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(memberships) => memberships,
        Err(e) => return Err(PulsarrError::validation_error(e)),
    };

    let members: Vec<PulsarrUser> = match pulsarr_user::get_by_ids(
        &memberships
            .iter()
            .map(|m| m.pulsarr_user_id)
            .collect::<Vec<i32>>(),
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(members) => members,
        Err(e) => return Err(PulsarrError::validation_error(e)),
    };

    let member_dtos = memberships.iter().map(|m| {
        let user = members
            .iter()
            .find(|u| u.pulsarr_user_id == m.pulsarr_user_id)
            .unwrap();
        return group_member_dto::to_dto(id, m, user);
    });

    let group = GroupDTO {
        members: Some(member_dtos.collect::<Vec<group_member_dto::GroupMemberDTO>>()),
        ..group_dto::to_dto(&pg, Some(&rs), Some(parameters))
    };

    Ok(Json(group))
}

/// # Get all groups
#[openapi(tag = "Group")]
#[post("/", format = "application/json", data = "<get_request>")]
async fn get_all_groups(
    state: &State<PostgresState>,
    get_request: Json<GetRequest>,
    _api_user: ApiKey,
) -> PulsarrResult<Vec<GroupDTO>> {
    let req = get_request.into_inner();
    match data_wrangler::get_all::<PulsarrGroup>(&state.pool, req.take_size, req.offset)
        .await
    {
        Ok(groups) => Ok(Json(
            groups
                .iter()
                .map(|group| group_dto::to_dto(group, None, None))
                .collect::<Vec<GroupDTO>>(),
        )),
        Err(e) => Err(e),
    }
}

/// # Create Group
#[openapi(tag = "Group")]
#[post("/create", format = "application/json", data = "<group_dto>")]
async fn create_group(
    state: &State<PostgresState>,
    group_dto: Json<GroupDTO>,
    _api_user: ApiKey,
) -> PulsarrResult<GroupDTO> {
    let mut group = group_dto.into_inner();
    let ApiKey(user_id) = _api_user;
    group.created_by_user_id = Some(user_id);
    if group.rating_system_id == 0 {
        match group.rating_system {
            Some(rsd) => {
                match data_wrangler::add(rating_system_dto::to_model(&rsd), &state.pool).await {
                    Ok(rating_system) => {
                        group.rating_system_id = rating_system.rating_system_id;
                        group.rating_system = Some(rating_system_dto::to_dto(&rating_system, None));
                    }
                    Err(e) => return Err(e),
                }

                // Map DTOs to models
                let parameter_models: Vec<RatingSystemParameter> = rsd.parameters
                    .map(|params| {
                        params.iter().map(|p| {
                            let mut dto = p.clone();
                            dto.rating_system_id = group.rating_system_id;
                            rating_system_parameter_dto::to_model(&dto)
                        }).collect()
                    })
                    .unwrap_or_default();

                // Persist models
                for parameter in parameter_models {
                    if let Err(e) = data_wrangler::add(parameter, &state.pool).await {
                        return Err(e);
                    }
                }
            }
            None => return Err(PulsarrError::missing_data("Rating System".to_string())),
        }
    }

    match data_wrangler::add(group_dto::to_model(&group), &state.pool).await {
        Ok(r) => {
            group.pulsarr_group_id = r.pulsarr_group_id;

            join(state, r.pulsarr_group_id, user_id, OWNER_ROLE.to_string()).await.expect("Failed to join created group as owner");

            Ok(Json(group))
        }
        Err(e) => Err(e),
    }
}

/// # Get the group membership types
#[openapi(tag = "Group")]
#[get("/membershipTypes")]
async fn get_membership_types() -> PulsarrResult<Vec<String>> {
    let mut membership_types = vec![];

    for typ in MEMBERSHIP_TYPE {
        membership_types.push(typ.to_owned());
    }

    Ok(Json(membership_types))
}

/// # Join group
#[openapi(tag = "Group")]
#[post("/join/<group_id>")]
async fn join_group(
    state: &State<PostgresState>,
    group_id: i32,
    api_user: ApiKey,
) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    // Setting to Member by default?
    join(state, group_id, user_id, MEMBER_ROLE.to_string()).await
}

/// # Leave group (Owner cannot leave — must transfer ownership first)
#[openapi(tag = "Group")]
#[post("/leave/<group_id>")]
async fn leave_group(
    state: &State<PostgresState>,
    group_id: i32,
    api_user: ApiKey,
) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;

    // Prevent Owner from leaving
    if let Some(membership) = authorization::get_membership(&state.pool, group_id, user_id).await {
        if membership.group_role == OWNER_ROLE {
            return Err(PulsarrError::forbidden("Owner cannot leave the group. Transfer ownership first."));
        }
    }

    match user_group::leave(group_id, user_id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}

async fn join(state: &State<PostgresState>, group_id: i32, user_id: i32, role: String) -> PulsarrResult<bool> {
    match user_group::join(group_id, user_id, role)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}

#[openapi(tag = "Group")]
#[get("/search?<name>")]
async fn search_groups(state: &State<PostgresState>, name: Option<&str>, api_user: ApiKey) -> PulsarrResult<Vec<GroupDTO>> {
    let ApiKey(user_id) = api_user;
    match pulsarr_group::get_by_name(name, user_id).fetch_all(&state.pool).await {
        Ok(groups) => Ok(Json(groups.iter().map(|group| group_dto::to_dto(group, None, None)).collect::<Vec<GroupDTO>>())),
        Err(e) => Err(PulsarrError::validation_error(e))
    }
}

#[openapi(tag = "Group")]
#[get("/myGroups")]
async fn my_groups(state: &State<PostgresState>, api_user: ApiKey) -> PulsarrResult<Vec<GroupDTO>> {
    let ApiKey(user_id) = api_user;
    match pulsarr_group::get_my_groups(&user_id).fetch_all(&state.pool).await {
        Ok(groups) => Ok(Json(groups.iter().map(|group| group_dto::to_dto(group, None, None)).collect::<Vec<GroupDTO>>())),
        Err(e) => Err(PulsarrError::validation_error(e))
    }
}

/// # Get all public groups (no auth required)
#[openapi(tag = "Group")]
#[get("/public")]
async fn get_public_groups_endpoint(
    state: &State<PostgresState>,
) -> PulsarrResult<Vec<GroupDTO>> {
    match pulsarr_group::get_public_groups::<PulsarrGroup>()
        .fetch_all(&state.pool)
        .await
    {
        Ok(groups) => Ok(Json(
            groups
                .iter()
                .map(|group| group_dto::to_dto(group, None, None))
                .collect(),
        )),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}

/// # Get group preview by id (no auth required)
#[openapi(tag = "Group")]
#[get("/preview/<id>")]
async fn get_group_preview_endpoint(
    state: &State<PostgresState>,
    id: i32,
) -> PulsarrResult<GroupDTO> {
    match pulsarr_group::get_group_preview::<PulsarrGroup>(id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(pg)) => Ok(Json(group_dto::to_dto(&pg, None, None))),
        Ok(None) => Err(PulsarrError::missing_data("Group".to_string())),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}

/// # Kick a member from a group (requires Admin+ role, actor must outrank target)
#[openapi(tag = "Group")]
#[post("/kick/<group_id>/<target_user_id>")]
async fn kick_member(
    state: &State<PostgresState>,
    group_id: i32,
    target_user_id: i32,
    api_user: ApiKey,
) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    let actor = authorization::require_role(&state.pool, group_id, user_id, ADMIN_ROLE).await?;

    let target = match authorization::get_membership(&state.pool, group_id, target_user_id).await {
        Some(t) => t,
        None => return Err(PulsarrError::validation_error("Target user is not a member of this group")),
    };

    if !authorization::can_act_on(&actor.group_role, &target.group_role) {
        return Err(PulsarrError::forbidden("You cannot kick a member with equal or higher role"));
    }

    match user_group::remove_member(group_id, target_user_id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}

/// # Change a member's role (requires Admin+ role, cannot set to Owner)
#[openapi(tag = "Group")]
#[post("/changeRole/<group_id>/<target_user_id>/<new_role>")]
async fn change_role(
    state: &State<PostgresState>,
    group_id: i32,
    target_user_id: i32,
    new_role: &str,
    api_user: ApiKey,
) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    let actor = authorization::require_role(&state.pool, group_id, user_id, ADMIN_ROLE).await?;

    if new_role == OWNER_ROLE {
        return Err(PulsarrError::forbidden("Cannot assign Owner role. Use transfer ownership instead."));
    }

    // Validate the new role is a known role
    if !MEMBERSHIP_TYPE.contains(&new_role) {
        return Err(PulsarrError::validation_error("Invalid role"));
    }

    let target = match authorization::get_membership(&state.pool, group_id, target_user_id).await {
        Some(t) => t,
        None => return Err(PulsarrError::validation_error("Target user is not a member of this group")),
    };

    if !authorization::can_act_on(&actor.group_role, &target.group_role) {
        return Err(PulsarrError::forbidden("You cannot change the role of a member with equal or higher role"));
    }

    // Admins can only set roles below their own level
    if authorization::role_level(new_role) >= authorization::role_level(&actor.group_role) {
        return Err(PulsarrError::forbidden("You cannot assign a role equal to or above your own"));
    }

    match user_group::update_role(group_id, target_user_id, new_role.to_string())
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}

/// # Transfer group ownership (requires Owner role)
#[openapi(tag = "Group")]
#[post("/transferOwnership/<group_id>/<new_owner_id>")]
async fn transfer_ownership(
    state: &State<PostgresState>,
    group_id: i32,
    new_owner_id: i32,
    api_user: ApiKey,
) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    authorization::require_role(&state.pool, group_id, user_id, OWNER_ROLE).await?;

    // Verify target is a member
    if authorization::get_membership(&state.pool, group_id, new_owner_id).await.is_none() {
        return Err(PulsarrError::validation_error("Target user is not a member of this group"));
    }

    // Promote new owner
    if let Err(e) = user_group::update_role(group_id, new_owner_id, OWNER_ROLE.to_string())
        .fetch_one(&state.pool)
        .await
    {
        return Err(PulsarrError::validation_error(e));
    }

    // Demote current owner to Admin
    match user_group::update_role(group_id, user_id, ADMIN_ROLE.to_string())
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e)),
    }
}
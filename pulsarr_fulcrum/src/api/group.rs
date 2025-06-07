use crate::api::dtos::get_request::GetRequest;
use crate::api::dtos::group_dto::GroupDTO;
use crate::api::dtos::{group_dto, rating_system_dto, rating_system_parameter_dto};
use crate::api::guards::api_key::ApiKey;
use crate::data::data_wrangler;
use crate::data::models::pulsarr_group::{PulsarrGroup, PRIVACY_TYPE};
use crate::data::models::rating_system::RatingSystem;
use crate::data::models::rating_system_parameter;
use crate::data::models::user_group;
use crate::error::PulsarrError;
use crate::{PostgresState, PulsarrResult};
use rocket::serde::json::Json;
use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use crate::data::models::user_group::MEMBERSHIP_TYPE;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: update_group, delete_group, get_pulsarr_group,
        get_all_groups, get_privacy_types, create_group, get_membership_types, join_group, leave_group]
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

/// # Update group
#[openapi(tag = "Group")]
#[post("/update", format = "application/json", data = "<group>")]
async fn update_group(state: &State<PostgresState>, group: Json<GroupDTO>, _api_user: ApiKey) -> PulsarrResult<GroupDTO> {
    match data_wrangler::update(group_dto::to_model(&group), &state.pool).await {
        Ok(r) => Ok(Json(group_dto::to_dto(&r, None, None))),
        Err(e) => Err(e)
    }
}

/// # Delete group
#[openapi(tag = "Group")]
#[delete("/delete/<id>")]
async fn delete_group(state: &State<PostgresState>, id: i32, _api_user: ApiKey) -> PulsarrResult<bool> {
    match data_wrangler::delete::<PulsarrGroup>(id, &state.pool).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(e)
    }
}

/// # Get a group by id
#[openapi(tag = "Group")]
#[get("/<id>")]
async fn get_pulsarr_group(state: &State<PostgresState>, id: i32, _api_user: ApiKey) -> PulsarrResult<GroupDTO> {
    match data_wrangler::get_by_id::<PulsarrGroup>(id, &state.pool).await {
        Ok(pg) => {
            match data_wrangler::get_by_id::<RatingSystem>(pg.rating_system_id, &state.pool).await {
                Ok(rs) => {
                    match rating_system_parameter::get_by_rating_system_id(rs.rating_system_id).fetch_all(&state.pool).await {
                        Ok(parameters) => {
                            let group = group_dto::to_dto(&pg, Some(rs), Some(parameters));
                            Ok(Json(group))
                        },
                        Err(e) => Err(PulsarrError::validation_error(e))
                    }
                },
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

/// # Get all groups
#[openapi(tag = "Group")]
#[post("/", format = "application/json", data = "<get_request>")]
async fn get_all_groups(state: &State<PostgresState>, get_request: Json<GetRequest>, _api_user: ApiKey) -> PulsarrResult<Vec<GroupDTO>> {
    match data_wrangler::get_all::<PulsarrGroup>(&state.pool, get_request.into_inner().take_size).await {
        Ok(groups) => Ok(Json(groups.iter().map(|group| group_dto::to_dto(group, None, None)).collect::<Vec<GroupDTO>>())),
        Err(e) => Err(e)
    }
}

/// # Create Group
#[openapi(tag = "Group")]
#[post("/create", format = "application/json", data = "<group_dto>")]
async fn create_group(state: &State<PostgresState>, group_dto: Json<GroupDTO>, _api_user: ApiKey) -> PulsarrResult<GroupDTO> {

    let mut group = group_dto.into_inner();
    if group.rating_system_id == 0 {
        match group.rating_system {
            Some(rsd) => {
                match data_wrangler::add(rating_system_dto::to_model(&rsd), &state.pool).await {
                    Ok(rating_system) => {
                        group.rating_system_id = rating_system.rating_system_id;
                        group.rating_system = Some(rating_system_dto::to_dto(rating_system, None));
                    },
                    Err(e) => return Err(e)
                }

                for mut parameter in rsd.parameters {
                    parameter.rating_system_id = group.rating_system_id;
                    match data_wrangler::add(rating_system_parameter_dto::to_model(&parameter), &state.pool).await {
                        Ok(rating_system_parameter) => {
                            parameter.rating_system_parameter_id = rating_system_parameter.rating_system_parameter_id;
                        },
                        Err(e) => return Err(e)
                    }
                }

            },
            None =>
                return Err(PulsarrError::missing_data("Rating System".to_string()))
        }
    }


    match data_wrangler::add(group_dto::to_model(&group), &state.pool).await {
        Ok(r) => {
            group.pulsarr_group_id = r.pulsarr_group_id;
            Ok(Json(group))
        },
        Err(e) => Err(e)
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
async fn join_group(state: &State<PostgresState>, group_id: i32, api_user: ApiKey) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    // Setting to Member by default?
    match user_group::join(group_id, user_id, "Member".to_string()).fetch_optional(&state.pool).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e))
    }
}

/// # Leave group
#[openapi(tag = "Group")]
#[post("/leave/<group_id>")]
async fn leave_group(state: &State<PostgresState>, group_id: i32, api_user: ApiKey) -> PulsarrResult<bool> {
    let ApiKey(user_id) = api_user;
    match user_group::leave(group_id, user_id).fetch_optional(&state.pool).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(PulsarrError::validation_error(e))
    }
}
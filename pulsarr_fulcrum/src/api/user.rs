use rocket::{get, post, State};
use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::PostgresState;
use crate::data::{ data_wrangler, models::pulsarr_user::PulsarrUser };

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_pulsarr_user, add_user]
}

/// # Get a user by id
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_pulsarr_user(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<PulsarrUser> {
    let rating_system =
        sqlx::query_as::<_, PulsarrUser>("select * from pulsarr_user where pulsarr_user_id = $1")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .unwrap();

    Ok(Json(rating_system))
}

/// # Add a user
#[openapi(tag = "User")]
#[post("/", format = "application/json", data = "<user>")]
async fn add_user(
    state: &State<PostgresState>,
    user: Json<PulsarrUser>,
) -> crate::PulsarrResult<bool> {
    data_wrangler::add(user.into_inner(), &state.pool).await
}
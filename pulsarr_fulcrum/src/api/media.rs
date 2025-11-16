use rocket::{delete, get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::api::dtos::get_request::GetRequest;
use crate::api::dtos::group_dto::GroupDTO;

use crate::api::dtos::{
    media_dto
};
use crate::api::guards::api_key::ApiKey;
use crate::{PostgresState, PulsarrResult};
use rocket::serde::json::Json;
use crate::api::dtos::media_dto::MediaDTO;
use crate::musicbrainz_client::Artist;

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: search_media]
}

/**
    Endpoint for getting media types
    Endpoint for searching media
*/

#[openapi(tag = "Media")]
#[get("/search?<media_type>&<search_param>")]
async fn search_media(
    state: &State<PostgresState>,
    media_type: Option<String>,
    search_param: Option<String>,
    api_user: ApiKey) -> PulsarrResult<MediaDTO> {

    let res = MediaDTO {
        media_id: 0,
        album_name: "".to_string(),
        artist: Artist {
            id: "".to_string(),
            name: "".to_string(),
            sort_name: None,
            artist_type: None,
            disambiguation: None,
            country: None,
            life_span: None,
        },
        release_date: Default::default(),
        tracks: vec![],
    };

    PulsarrResult::Ok(Json(res))
}
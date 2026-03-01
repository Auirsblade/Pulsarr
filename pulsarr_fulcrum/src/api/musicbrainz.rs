use rocket::{get, State};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::musicbrainz_client::{MusicBrainzClient, ArtistSearchResponse, ReleaseSearchResponse, RecordingSearchResponse, ReleaseGroupSearchResponse, Artist, Release, Recording, ReleaseGroup, CoverArtArchive, CoverArtInfo};
use crate::error::PulsarrError;

pub struct MusicBrainzState {
    pub client: MusicBrainzClient,
}

/// Search for artists on MusicBrainz
#[openapi(tag = "MusicBrainz")]
#[get("/search/artist?<query>&<limit>&<offset>")]
async fn search_artists(
    state: &State<MusicBrainzState>,
    query: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Json<ArtistSearchResponse>, PulsarrError> {
    match state.client.search_artists(&query, limit, offset).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get artist by MBID
#[openapi(tag = "MusicBrainz")]
#[get("/artist/<mbid>")]
async fn get_artist(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<Artist>, PulsarrError> {
    match state.client.get_artist(&mbid, None).await {
        Ok(artist) => Ok(Json(artist)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Search for releases on MusicBrainz
#[openapi(tag = "MusicBrainz")]
#[get("/search/release?<query>&<limit>&<offset>")]
async fn search_releases(
    state: &State<MusicBrainzState>,
    query: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Json<ReleaseSearchResponse>, PulsarrError> {
    match state.client.search_releases(&query, limit, offset).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get release by MBID
#[openapi(tag = "MusicBrainz")]
#[get("/release/<mbid>")]
async fn get_release(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<Release>, PulsarrError> {
    match state.client.get_release(&mbid, None).await {
        Ok(release) => Ok(Json(release)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Search for recordings on MusicBrainz
#[openapi(tag = "MusicBrainz")]
#[get("/search/recording?<query>&<limit>&<offset>")]
async fn search_recordings(
    state: &State<MusicBrainzState>,
    query: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Json<RecordingSearchResponse>, PulsarrError> {
    match state.client.search_recordings(&query, limit, offset).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get recording by MBID
#[openapi(tag = "MusicBrainz")]
#[get("/recording/<mbid>")]
async fn get_recording(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<Recording>, PulsarrError> {
    match state.client.get_recording(&mbid, None).await {
        Ok(recording) => Ok(Json(recording)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get cover art for a release
#[openapi(tag = "MusicBrainz")]
#[get("/release/<mbid>/cover-art")]
async fn get_release_cover_art(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<CoverArtArchive>, PulsarrError> {
    match state.client.get_release_cover_art(&mbid).await {
        Ok(cover_art) => Ok(Json(cover_art)),
        Err(e) => Err(PulsarrError {
            err: "Cover Art Archive API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get cover art info for a release (convenience method with front/back URLs)
#[openapi(tag = "MusicBrainz")]
#[get("/release/<mbid>/cover-art-info")]
async fn get_release_cover_art_info(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<CoverArtInfo>, PulsarrError> {
    match state.client.get_release_cover_art_info(&mbid).await {
        Ok(info) => Ok(Json(info)),
        Err(e) => Err(PulsarrError {
            err: "Cover Art Archive API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get cover art for a release group
#[openapi(tag = "MusicBrainz")]
#[get("/release-group/<mbid>/cover-art")]
async fn get_release_group_cover_art(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<CoverArtArchive>, PulsarrError> {
    match state.client.get_release_group_cover_art(&mbid).await {
        Ok(cover_art) => Ok(Json(cover_art)),
        Err(e) => Err(PulsarrError {
            err: "Cover Art Archive API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Search for release groups on MusicBrainz
#[openapi(tag = "MusicBrainz")]
#[get("/search/release-group?<query>&<limit>&<offset>")]
async fn search_release_groups(
    state: &State<MusicBrainzState>,
    query: &str,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Json<ReleaseGroupSearchResponse>, PulsarrError> {
    match state.client.search_release_groups(&query, limit, offset).await {
        Ok(mut response) => {
            response.release_groups.retain(|rg| {
                let is_album_or_ep = matches!(
                    rg.primary_type.as_deref(),
                    Some("Album") | Some("EP")
                );
                let is_mixtape = rg.secondary_types.as_ref().is_some_and(|types| {
                    types.iter().any(|t| t == "Mixtape/Street")
                });
                is_album_or_ep || is_mixtape
            });
            Ok(Json(response))
        }
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get release group by MBID
#[openapi(tag = "MusicBrainz")]
#[get("/release-group/<mbid>")]
async fn get_release_group(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<ReleaseGroup>, PulsarrError> {
    match state.client.get_release_group(&mbid, None).await {
        Ok(release_group) => Ok(Json(release_group)),
        Err(e) => Err(PulsarrError {
            err: "MusicBrainz API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

/// Get cover art info for a release group (convenience method with front/back URLs)
#[openapi(tag = "MusicBrainz")]
#[get("/release-group/<mbid>/cover-art-info")]
async fn get_release_group_cover_art_info(
    state: &State<MusicBrainzState>,
    mbid: &str,
) -> Result<Json<CoverArtInfo>, PulsarrError> {
    match state.client.get_release_group_cover_art_info(&mbid).await {
        Ok(info) => Ok(Json(info)),
        Err(e) => Err(PulsarrError {
            err: "Cover Art Archive API Error".to_string(),
            msg: Some(e.to_string()),
            http_status_code: 500,
        }),
    }
}

pub fn get_routes_and_docs(settings: &rocket_okapi::settings::OpenApiSettings) -> (Vec<rocket::Route>, rocket_okapi::okapi::openapi3::OpenApi) {
    rocket_okapi::openapi_get_routes_spec![
        settings:
        search_artists,
        get_artist,
        search_releases,
        get_release,
        search_recordings,
        get_recording,
        get_release_cover_art,
        get_release_cover_art_info,
        get_release_group_cover_art,
        search_release_groups,
        get_release_group,
        get_release_group_cover_art_info
    ]
}
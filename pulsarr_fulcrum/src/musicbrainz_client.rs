use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;

const MUSICBRAINZ_API_BASE: &str = "https://musicbrainz.org/ws/2";
const USER_AGENT: &str = "Pulsarr/0.1.0 (https://github.com/Auirsblade/Pulsarr)";

#[derive(Debug, Clone)]
pub struct MusicBrainzClient {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(rename = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: Option<String>,
    pub disambiguation: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "life-span")]
    pub life_span: Option<LifeSpan>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct LifeSpan {
    pub begin: Option<String>,
    pub end: Option<String>,
    pub ended: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Release {
    pub id: String,
    pub title: String,
    pub status: Option<String>,
    pub date: Option<String>,
    #[serde(rename = "release-group")]
    pub release_group: Option<ReleaseGroup>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ReleaseGroup {
    pub id: String,
    pub title: Option<String>,
    #[serde(rename = "primary-type")]
    pub primary_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ArtistCredit {
    pub name: Option<String>,
    pub artist: Option<Artist>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub length: Option<u64>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResponse<T> {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub artists: Option<Vec<T>>,
    pub releases: Option<Vec<T>>,
    pub recordings: Option<Vec<T>>,
    #[serde(rename = "release-groups")]
    pub release_groups: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArtistSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RecordingSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub recordings: Vec<Recording>,
}

impl MusicBrainzClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: MUSICBRAINZ_API_BASE.to_string(),
        }
    }

    /// Search for artists by query string
    pub async fn search_artists(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ArtistSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/artist", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<ArtistSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get artist by MBID (MusicBrainz ID)
    pub async fn get_artist(&self, mbid: &str) -> Result<Artist, reqwest::Error> {
        let url = format!("{}/artist/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json");

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Artist>()
            .await?;

        Ok(response)
    }

    /// Search for releases by query string
    pub async fn search_releases(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ReleaseSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/release", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<ReleaseSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get release by MBID
    pub async fn get_release(&self, mbid: &str) -> Result<Release, reqwest::Error> {
        let url = format!("{}/release/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json");

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Release>()
            .await?;

        Ok(response)
    }

    /// Search for recordings by query string
    pub async fn search_recordings(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<RecordingSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/recording", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<RecordingSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get recording by MBID
    pub async fn get_recording(&self, mbid: &str) -> Result<Recording, reqwest::Error> {
        let url = format!("{}/recording/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json");

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Recording>()
            .await?;

        Ok(response)
    }

    /// Search by artist name (convenience method)
    pub async fn search_artist_by_name(&self, name: &str) -> Result<ArtistSearchResponse, reqwest::Error> {
        let query = format!("artist:{}", name);
        self.search_artists(&query, Some(10), None).await
    }

    /// Search releases by artist name
    pub async fn search_releases_by_artist(&self, artist: &str) -> Result<ReleaseSearchResponse, reqwest::Error> {
        let query = format!("artist:{}", artist);
        self.search_releases(&query, Some(25), None).await
    }

    /// Search recordings by track name
    pub async fn search_recording_by_track(&self, track: &str) -> Result<RecordingSearchResponse, reqwest::Error> {
        let query = format!("recording:{}", track);
        self.search_recordings(&query, Some(25), None).await
    }
}

impl Default for MusicBrainzClient {
    fn default() -> Self {
        Self::new()
    }
}
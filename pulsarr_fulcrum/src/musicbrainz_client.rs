use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;

const MUSICBRAINZ_API_BASE: &str = "https://musicbrainz.org/ws/2";
const COVERART_API_BASE: &str = "https://coverartarchive.org";
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
    pub releases: Option<Vec<Release>>,
    #[serde(rename = "release-groups")]
    pub release_groups: Option<Vec<ReleaseGroup>>,
    pub recordings: Option<Vec<Recording>>,
    pub works: Option<Vec<Work>>,
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
    pub country: Option<String>,
    pub barcode: Option<String>,
    pub media: Option<Vec<Media>>,
    #[serde(rename = "label-info")]
    pub label_info: Option<Vec<LabelInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ReleaseGroup {
    pub id: String,
    pub title: Option<String>,
    #[serde(rename = "primary-type")]
    pub primary_type: Option<String>,
    #[serde(rename = "secondary-types")]
    pub secondary_types: Option<Vec<String>>,
    #[serde(rename = "first-release-date")]
    pub first_release_date: Option<String>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub releases: Option<Vec<Release>>,
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
    pub releases: Option<Vec<Release>>,
    #[serde(rename = "isrcs")]
    pub isrcs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Work {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    pub language: Option<String>,
    pub disambiguation: Option<String>,
    #[serde(rename = "iswcs")]
    pub iswcs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Label {
    pub id: String,
    pub name: String,
    #[serde(rename = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(rename = "label-code")]
    pub label_code: Option<u32>,
    #[serde(rename = "type")]
    pub label_type: Option<String>,
    pub disambiguation: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "life-span")]
    pub life_span: Option<LifeSpan>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Area {
    pub id: String,
    pub name: String,
    #[serde(rename = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(rename = "type")]
    pub area_type: Option<String>,
    #[serde(rename = "iso-3166-1-codes")]
    pub iso_3166_1_codes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Media {
    pub format: Option<String>,
    #[serde(rename = "track-count")]
    pub track_count: Option<u32>,
    pub tracks: Option<Vec<Track>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Track {
    pub id: String,
    pub number: Option<String>,
    pub title: String,
    pub length: Option<u64>,
    pub recording: Option<Recording>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct LabelInfo {
    #[serde(rename = "catalog-number")]
    pub catalog_number: Option<String>,
    pub label: Option<Label>,
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
pub struct ReleaseGroupSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    #[serde(rename = "release-groups")]
    pub release_groups: Vec<ReleaseGroup>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub works: Vec<Work>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LabelSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub labels: Vec<Label>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AreaSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub areas: Vec<Area>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BrowseReleasesResponse {
    pub created: Option<String>,
    #[serde(rename = "release-count")]
    pub release_count: u32,
    #[serde(rename = "release-offset")]
    pub release_offset: u32,
    pub releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BrowseReleaseGroupsResponse {
    pub created: Option<String>,
    #[serde(rename = "release-group-count")]
    pub release_group_count: u32,
    #[serde(rename = "release-group-offset")]
    pub release_group_offset: u32,
    #[serde(rename = "release-groups")]
    pub release_groups: Vec<ReleaseGroup>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BrowseRecordingsResponse {
    pub created: Option<String>,
    #[serde(rename = "recording-count")]
    pub recording_count: u32,
    #[serde(rename = "recording-offset")]
    pub recording_offset: u32,
    pub recordings: Vec<Recording>,
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Url {
    pub id: String,
    pub resource: String,
    #[serde(rename = "relation-list")]
    pub relation_list: Option<Vec<Relation>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Relation {
    #[serde(rename = "type")]
    pub relation_type: Option<String>,
    #[serde(rename = "type-id")]
    pub type_id: Option<String>,
    pub direction: Option<String>,
    pub artist: Option<Artist>,
    pub release: Option<Release>,
    pub recording: Option<Recording>,
    pub work: Option<Work>,
    pub label: Option<Label>,
    pub url: Option<UrlResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct UrlResource {
    pub id: Option<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UrlSearchResponse {
    pub created: Option<String>,
    pub count: u32,
    pub offset: u32,
    pub urls: Vec<Url>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CoverArtImage {
    pub id: u64,
    pub image: String,
    pub thumbnails: Option<CoverArtThumbnails>,
    pub front: bool,
    pub back: bool,
    pub types: Vec<String>,
    pub comment: Option<String>,
    pub approved: Option<bool>,
    pub edit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CoverArtThumbnails {
    #[serde(rename = "250")]
    pub size_250: Option<String>,
    #[serde(rename = "500")]
    pub size_500: Option<String>,
    #[serde(rename = "1200")]
    pub size_1200: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CoverArtArchive {
    pub images: Vec<CoverArtImage>,
    pub release: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CoverArtInfo {
    pub front: Option<String>,
    pub back: Option<String>,
    pub front_thumbnail_small: Option<String>,
    pub front_thumbnail_large: Option<String>,
    pub back_thumbnail_small: Option<String>,
    pub back_thumbnail_large: Option<String>,
    pub all_images: Vec<CoverArtImage>,
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
    /// inc can include: recordings, releases, release-groups, works
    pub async fn get_artist(&self, mbid: &str, inc: Option<Vec<&str>>) -> Result<Artist, reqwest::Error> {
        let url = format!("{}/artist/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        if let Some(inc) = inc {
            params.insert("inc", inc.join("+"));
        } else {
            params.insert("inc", "recordings+releases+release-groups+works".to_string());
        }

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
    /// inc can include: artists, labels, recordings, release-groups, media
    pub async fn get_release(&self, mbid: &str, inc: Option<Vec<&str>>) -> Result<Release, reqwest::Error> {
        let url = format!("{}/release/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        if let Some(inc) = inc {
            params.insert("inc", inc.join("+"));
        } else {
            params.insert("inc", ["artist-credits", "labels", "recordings", "release-groups", "media"].join("+"));
        }

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
    /// inc can include: artist-credits, releases, isrcs
    pub async fn get_recording(&self, mbid: &str, inc: Option<Vec<&str>>) -> Result<Recording, reqwest::Error> {
        let url = format!("{}/recording/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        if let Some(inc) = inc {
            params.insert("inc", inc.join("+"));
        } else {
            params.insert("inc", ["artist-credits", "releases", "isrcs"].join("+"));
        }

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

    /// Search for release groups by query string
    pub async fn search_release_groups(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ReleaseGroupSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/release-group", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<ReleaseGroupSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get release group by MBID
    /// inc can include: artist-credits, releases
    pub async fn get_release_group(&self, mbid: &str, inc: Option<Vec<&str>>) -> Result<ReleaseGroup, reqwest::Error> {
        let url = format!("{}/release-group/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        if let Some(inc) = inc {
            params.insert("inc", inc.join("+"));
        }
        else {
            params.insert("inc", ["artist-credits", "releases"].join("+"));
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<ReleaseGroup>()
            .await?;

        Ok(response)
    }

    /// Search for works by query string
    pub async fn search_works(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<WorkSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/work", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<WorkSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get work by MBID
    pub async fn get_work(&self, mbid: &str) -> Result<Work, reqwest::Error> {
        let url = format!("{}/work/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Work>()
            .await?;

        Ok(response)
    }

    /// Search for labels by query string
    pub async fn search_labels(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<LabelSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/label", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<LabelSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get label by MBID
    pub async fn get_label(&self, mbid: &str) -> Result<Label, reqwest::Error> {
        let url = format!("{}/label/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Label>()
            .await?;

        Ok(response)
    }

    /// Search for areas by query string
    pub async fn search_areas(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<AreaSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/area", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<AreaSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get area by MBID
    pub async fn get_area(&self, mbid: &str) -> Result<Area, reqwest::Error> {
        let url = format!("{}/area/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Area>()
            .await?;

        Ok(response)
    }

    /// Browse releases by artist MBID
    pub async fn browse_releases_by_artist(
        &self,
        artist_mbid: &str,
        limit: Option<u32>,
        offset: Option<u32>,
        release_type: Option<Vec<&str>>,
        release_status: Option<Vec<&str>>,
    ) -> Result<BrowseReleasesResponse, reqwest::Error> {
        let url = format!("{}/release", self.base_url);
        let mut params = HashMap::new();
        params.insert("artist", artist_mbid.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }
        if let Some(types) = release_type {
            params.insert("type", types.join("|"));
        }
        if let Some(statuses) = release_status {
            params.insert("status", statuses.join("|"));
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<BrowseReleasesResponse>()
            .await?;

        Ok(response)
    }

    /// Browse release groups by artist MBID
    pub async fn browse_release_groups_by_artist(
        &self,
        artist_mbid: &str,
        limit: Option<u32>,
        offset: Option<u32>,
        release_type: Option<Vec<&str>>,
    ) -> Result<BrowseReleaseGroupsResponse, reqwest::Error> {
        let url = format!("{}/release-group", self.base_url);
        let mut params = HashMap::new();
        params.insert("artist", artist_mbid.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }
        if let Some(types) = release_type {
            params.insert("type", types.join("|"));
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<BrowseReleaseGroupsResponse>()
            .await?;

        Ok(response)
    }

    /// Browse recordings by artist MBID
    pub async fn browse_recordings_by_artist(
        &self,
        artist_mbid: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<BrowseRecordingsResponse, reqwest::Error> {
        let url = format!("{}/recording", self.base_url);
        let mut params = HashMap::new();
        params.insert("artist", artist_mbid.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<BrowseRecordingsResponse>()
            .await?;

        Ok(response)
    }

    /// Browse releases by release group MBID
    pub async fn browse_releases_by_release_group(
        &self,
        release_group_mbid: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<BrowseReleasesResponse, reqwest::Error> {
        let url = format!("{}/release", self.base_url);
        let mut params = HashMap::new();
        params.insert("release-group", release_group_mbid.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<BrowseReleasesResponse>()
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

    /// Search for URLs by query string
    pub async fn search_urls(
        &self,
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<UrlSearchResponse, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("query", query.to_string());
        params.insert("fmt", "json".to_string());

        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let url = format!("{}/url", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<UrlSearchResponse>()
            .await?;

        Ok(response)
    }

    /// Get URL by MBID
    /// inc can include: artist-rels, release-rels, recording-rels, work-rels, label-rels
    pub async fn get_url(&self, mbid: &str, inc: Option<Vec<&str>>) -> Result<Url, reqwest::Error> {
        let url = format!("{}/url/{}", self.base_url, mbid);
        let mut params = HashMap::new();
        params.insert("fmt", "json".to_string());

        if let Some(inc) = inc {
            params.insert("inc", inc.join("+"));
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Url>()
            .await?;

        Ok(response)
    }

    /// Search for URLs by resource (convenience method)
    pub async fn search_url_by_resource(&self, resource: &str) -> Result<UrlSearchResponse, reqwest::Error> {
        let query = format!("url:{}", resource);
        self.search_urls(&query, Some(10), None).await
    }

    /// Get cover art for a release by MBID
    /// Returns all available cover art images with their metadata
    pub async fn get_release_cover_art(&self, release_mbid: &str) -> Result<CoverArtArchive, reqwest::Error> {
        let url = format!("{}/release/{}", COVERART_API_BASE, release_mbid);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<CoverArtArchive>()
            .await?;

        Ok(response)
    }

    /// Get cover art for a release group by MBID
    /// Returns all available cover art images with their metadata
    pub async fn get_release_group_cover_art(&self, release_group_mbid: &str) -> Result<CoverArtArchive, reqwest::Error> {
        let url = format!("{}/release-group/{}", COVERART_API_BASE, release_group_mbid);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<CoverArtArchive>()
            .await?;

        Ok(response)
    }

    /// Get front cover art image URL for a release
    pub async fn get_release_front_cover(&self, release_mbid: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/release/{}/front", COVERART_API_BASE, release_mbid);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        Ok(response.url().to_string())
    }

    /// Get back cover art image URL for a release
    pub async fn get_release_back_cover(&self, release_mbid: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/release/{}/back", COVERART_API_BASE, release_mbid);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        Ok(response.url().to_string())
    }

    /// Get front cover art image URL for a release group
    pub async fn get_release_group_front_cover(&self, release_group_mbid: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/release-group/{}/front", COVERART_API_BASE, release_group_mbid);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        Ok(response.url().to_string())
    }

    /// Get a specific cover art image by ID for a release
    pub async fn get_release_cover_art_by_id(&self, release_mbid: &str, image_id: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/release/{}/{}", COVERART_API_BASE, release_mbid, image_id);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        Ok(response.url().to_string())
    }

    /// Get thumbnail for a specific image size (250, 500, or 1200)
    pub async fn get_release_cover_art_thumbnail(
        &self, 
        release_mbid: &str, 
        image_id: &str, 
        size: u32
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}/release/{}/{}-{}", COVERART_API_BASE, release_mbid, image_id, size);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        Ok(response.url().to_string())
    }

    /// Convenience method to get all relevant cover art URLs for a release
    pub async fn get_release_cover_art_info(&self, release_mbid: &str) -> Result<CoverArtInfo, reqwest::Error> {
        match self.get_release_cover_art(release_mbid).await {
            Ok(archive) => {
                let front_image = archive.images.iter().find(|img| img.front);
                let back_image = archive.images.iter().find(|img| img.back);

                Ok(CoverArtInfo {
                    front: front_image.map(|img| img.image.clone()),
                    back: back_image.map(|img| img.image.clone()),
                    front_thumbnail_small: front_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.small.clone()),
                    front_thumbnail_large: front_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.large.clone()),
                    back_thumbnail_small: back_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.small.clone()),
                    back_thumbnail_large: back_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.large.clone()),
                    all_images: archive.images,
                })
            }
            Err(e) => Err(e),
        }
    }

    /// Convenience method to get all relevant cover art URLs for a release group
    pub async fn get_release_group_cover_art_info(&self, release_group_mbid: &str) -> Result<CoverArtInfo, reqwest::Error> {
        match self.get_release_group_cover_art(release_group_mbid).await {
            Ok(archive) => {
                let front_image = archive.images.iter().find(|img| img.front);
                let back_image = archive.images.iter().find(|img| img.back);

                Ok(CoverArtInfo {
                    front: front_image.map(|img| img.image.clone()),
                    back: back_image.map(|img| img.image.clone()),
                    front_thumbnail_small: front_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.small.clone()),
                    front_thumbnail_large: front_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.large.clone()),
                    back_thumbnail_small: back_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.small.clone()),
                    back_thumbnail_large: back_image
                        .and_then(|img| img.thumbnails.as_ref())
                        .and_then(|t| t.large.clone()),
                    all_images: archive.images,
                })
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for MusicBrainzClient {
    fn default() -> Self {
        Self::new()
    }
}
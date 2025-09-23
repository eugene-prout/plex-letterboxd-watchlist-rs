use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchMediaContainer {
    pub size: u32,
    pub identifier: String,
    #[serde(rename = "mediaTagPrefix")]
    pub media_tag_prefix: String,
    #[serde(rename = "mediaTagVersion")]
    pub media_tag_version: u64,
    #[serde(rename = "SearchResult")]
    pub search_result: Vec<SearchResultItem>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultItem {
    pub thumb: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub guid: String,
    pub name: String,
    pub year: u32,
    pub summary: String,
    #[serde(rename = "lifespanEnded")]
    pub lifespan_ended: bool,
}

#[derive(Debug, Deserialize)]
pub struct SearchRoot {
    #[serde(rename = "MediaContainer")]
    pub media_container: SearchMediaContainer,
}

#[derive(Debug, Deserialize)]
pub struct MediaRoot {
    #[serde(rename = "MediaContainer")]
    pub media_container: MediaContainer,
}

#[derive(Debug, Deserialize)]
pub struct MediaContainer {
    #[serde(rename = "Metadata", default)]
    pub metadata: Vec<MovieMetadata>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MovieMetadata {
    #[serde(rename = "ratingKey")]
    pub rating_key: String,
    pub key: String,
    pub title: String,
    pub summary: String,
}

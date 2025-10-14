use crate::error::{Error, SerdeSnafu};
use crate::util::{AlbumImage, Image, get_url};
use serde::{Deserialize, Deserializer};
use snafu::ResultExt;
use url::form_urlencoded::byte_serialize;

const SEARCH_ENDPOINT: &str = "https://bandcamp.com/api/fuzzysearch/2/app_autocomplete";

#[derive(Deserialize)]
struct SearchResult {
    results: Vec<SearchResultItem>,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SearchResultItem {
    #[serde(rename = "b")]
    Artist(SearchResultItemArtist),
    #[serde(rename = "a")]
    Album(SearchResultItemAlbum),
    #[serde(rename = "t")]
    Track(SearchResultItemTrack),
    #[serde(rename = "f")]
    Fan(SearchResultItemFan),
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct SearchResultItemArtist {
    #[serde(rename = "id")]
    pub artist_id: u64,
    #[serde(flatten)]
    pub image: Image,
    pub name: String,
    #[serde(default)]
    pub location: Option<String>,
    pub is_label: bool,
    #[serde(
        default,
        rename = "tag_names",
        deserialize_with = "crate::util::null_as_default"
    )]
    pub tags: Vec<String>,
    #[serde(rename = "genre_name")]
    pub genre: Option<String>,
    pub url: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct BandcampUrl {
    pub artist_url: String,
    pub item_url: String,
}

fn split_bandcamp_url<'de, D>(deserializer: D) -> Result<BandcampUrl, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;

    // Try to find the second occurrence of "https://"
    if let Some(pos) = s[8..].find("https://") {
        let split_at = pos + 8; // adjust index (skip first "https://")
        let item_url = s.split_off(split_at);
        Ok(BandcampUrl {
            artist_url: s,
            item_url,
        })
    } else {
        Err(serde::de::Error::custom("Expected two concatenated URLs"))
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct SearchResultItemAlbum {
    #[serde(rename = "id")]
    pub album_id: u64,
    #[serde(flatten)]
    pub image: AlbumImage,
    pub name: String,
    pub band_id: u64,
    pub band_name: String,
    #[serde(deserialize_with = "split_bandcamp_url")]
    pub url: BandcampUrl,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct SearchResultItemTrack {
    #[serde(rename = "id")]
    pub track_id: u64,
    #[serde(flatten)]
    pub image: AlbumImage,
    pub name: String,
    pub band_id: u64,
    pub band_name: String,
    pub album_id: Option<u64>,
    pub album_name: Option<String>,
    #[serde(deserialize_with = "split_bandcamp_url")]
    pub url: BandcampUrl,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct SearchResultItemFan {
    #[serde(rename = "id")]
    pub fan_id: u64,
    #[serde(flatten)]
    pub image: Image,
    pub name: String,
    pub username: String,
    pub collection_size: u64,
    pub genre_name: String,
    pub url: String,
}

pub async fn search(query: &str) -> Result<Vec<SearchResultItem>, Error> {
    let escaped_query: String = byte_serialize(query.as_bytes()).collect();
    let (result, _) = get_url(format!(
        "{}?q={}&param_with_locations=true",
        SEARCH_ENDPOINT, escaped_query
    ))
    .await?;
    let result: SearchResult = serde_json::from_str(&result).context(SerdeSnafu)?;
    Ok(result.results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_works() {
        search("Meschera").await.unwrap();
    }

    #[tokio::test]
    async fn test_foo() {
        search("foo").await.unwrap();
    }
}

use crate::error::{Error, SerdeSnafu};
use crate::util::get_url;
use serde::{Deserialize, Deserializer};
use snafu::ResultExt;
use std::io::Write;

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
pub struct SearchResultItemArtist {
    #[serde(rename = "id")]
    pub artist_id: u64,
    #[serde(flatten)]
    pub image: ImageId,
    pub name: String,
    #[serde(default)]
    pub location: Option<String>,
    pub is_label: bool,
    #[serde(default, rename = "tag_names")]
    pub tags: Vec<String>,
    #[serde(rename = "genre_name")]
    pub genre: String,
    pub url: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BandcampUrl {
    artist_url: String,
    item_url: String,
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
pub struct SearchResultItemAlbum {
    #[serde(rename = "id")]
    pub album_id: u64,
    #[serde(flatten)]
    pub image: ImageId,
    pub name: String,
    pub band_id: u64,
    pub band_name: String,
    #[serde(deserialize_with = "split_bandcamp_url")]
    url: BandcampUrl,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
pub struct SearchResultItemTrack {
    #[serde(rename = "id")]
    pub track_id: u64,
    #[serde(flatten)]
    pub image: ImageId,
    pub name: String,
    pub band_id: u64,
    pub band_name: String,
    pub album_id: Option<u64>,
    pub album_name: Option<String>,
    #[serde(deserialize_with = "split_bandcamp_url")]
    url: BandcampUrl,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
pub struct SearchResultItemFan {
    #[serde(rename = "id")]
    pub fan_id: u64,
    #[serde(flatten)]
    pub image: ImageId,
    pub name: String,
    pub username: String,
    pub collection_size: u64,
    pub genre_name: String,
    pub url: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
pub struct ImageId {
    #[serde(default)]
    image_id: Option<u64>,
    #[serde(default)]
    img_id: Option<u64>,
    #[serde(default)]
    art_id: Option<u64>,
}

impl ImageId {
    pub fn get_image_id(&self) -> Option<u64> {
        self.image_id.or(self.img_id.or(self.art_id))
    }

    pub fn get_url(&self) -> Option<String> {
        self.get_image_id()
            .map(|id| format!("https://f4.bcbits.com/img/{:010}_0.jpg", id))
    }
}

pub fn search(query: &str) -> Result<Vec<SearchResultItem>, Error> {
    let (result, _) = get_url(format!(
        "{}?q={}&param_with_locations=true",
        SEARCH_ENDPOINT, query
    ))?;
    let mut file = std::fs::File::create("result.json").unwrap();
    file.write_all(result.as_bytes()).unwrap();
    let result: SearchResult = serde_json::from_str(&result).context(SerdeSnafu)?;
    Ok(result.results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        search("Meschera").unwrap();
    }
}

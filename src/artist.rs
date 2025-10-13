use crate::error::*;
use crate::search::ImageId;
use crate::util::get_url;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use snafu::ResultExt;

const ARTISTS_URL: &'static str = "https://bandcamp.com/api/mobile/25/band_details";

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct Artist {
    pub id: u64,
    pub name: String,
    #[serde(rename = "bio_image_id")]
    pub image_id: Option<u64>,
    pub bio: Option<String>,
    #[serde(rename = "bandcamp_url")]
    pub url: String,
    pub sites: Vec<ArtistSite>,
    pub location: Option<String>,
    pub discography: Vec<ArtistDiscographyEntry>,
    /// artists are only present for labels
    #[serde(default)]
    pub artists: Vec<LabelArtist>,
    // TODO: Shows and merch
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct ArtistSite {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct ArtistDiscographyEntry {
    #[serde(rename = "item_id")]
    pub id: u64,
    pub item_type: ArtistDiscographyEntryType,
    /// Artist name of this specific release
    pub artist_name: Option<String>,
    /// Name of the band this is released under
    pub band_id: u64,
    pub band_name: String,
    pub title: String,
    #[serde(flatten)]
    pub image: ImageId,
    pub is_purchasable: bool,
    #[serde(with = "crate::date_serializer")]
    pub release_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq, eq_int))]
pub enum ArtistDiscographyEntryType {
    Album,
    Track,
}

/// Artist entry for a label
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq))]
pub struct LabelArtist {
    pub id: u64,
    pub name: String,
    #[serde(flatten)]
    pub image_id: ImageId,
    pub location: Option<String>,
}

pub fn fetch_artist(artist_id: u64) -> Result<Artist, Error> {
    let url = format!("{}?band_id={}", ARTISTS_URL, artist_id);
    let (content, _) = get_url(url)?;
    serde_json::from_str(&content).context(SerdeSnafu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grai() {
        fetch_artist(3250782803).unwrap();
    }

    #[test]
    fn test_meschera() {
        fetch_artist(3752216131).unwrap();
    }

    #[test]
    fn test_bergfried() {
        let result = fetch_artist(3899303380).unwrap();
        assert!(result.location.is_none());
    }

    #[test]
    fn test_napalm_records() {
        let result = fetch_artist(4115243786).unwrap();
        assert_eq!(result.name, "Napalm Records".to_string());
        assert!(!result.artists.is_empty());
    }
}

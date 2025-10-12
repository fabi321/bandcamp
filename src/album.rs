use crate::error::{Error, SerdeSnafu};
use crate::search::ImageId;
use crate::util::get_url;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use snafu::ResultExt;
use std::collections::HashMap;
use std::io::Write;

const ALBUMS_URL: &'static str = "https://bandcamp.com/api/mobile/25/tralbum_details";

/// Single track or album
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Album {
    /// Track or album id
    pub id: u64,
    /// Type of item
    #[serde(rename = "type")]
    pub item_type: AlbumType,
    pub title: String,
    #[serde(rename = "bandcamp_url")]
    pub url: String,
    #[serde(flatten)]
    pub image_id: ImageId,
    /// Band this belongs to
    pub band: AlbumBand,
    /// For a track belonging to an album
    pub album_id: Option<u64>,
    /// For a track belonging to an album
    pub album_title: Option<String>,
    /// About text for an album or track, explaining it a bit
    pub about: Option<String>,
    /// Credits for the album or track
    pub credits: Option<String>,
    pub tracks: Vec<AlbumTrack>,
    /// The track id of the featured track, if any
    pub featured_track: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub release_date: DateTime<Utc>,
    pub tags: Vec<AlbumTag>,
    pub free_download: bool,
    pub is_preorder: bool,
    #[serde(flatten)]
    pub purchase_options: PurchaseOptions,
    pub label: Option<String>,
    pub label_id: Option<u64>,
    pub num_downloadable_tracks: u64,
    pub merch_sold_out: Option<bool>,
    /// How often a user can stream a track before being prompted to buy it,
    /// see https://get.bandcamp.help/hc/en-us/articles/23020694060183-What-are-streaming-limits-on-Bandcamp
    pub streaming_limit: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum AlbumType {
    #[serde(rename = "a")]
    Album,
    #[serde(rename = "t")]
    Track,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AlbumBand {
    #[serde(rename = "band_id")]
    pub id: u64,
    pub name: String,
    #[serde(flatten)]
    pub image_id: ImageId,
    pub bio: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AlbumTag {
    pub name: String,
    #[serde(rename = "norm_name")]
    pub normalized_name: String,
    pub url: String,
    #[serde(rename = "isloc")]
    pub is_location: bool,
    #[serde(rename = "loc_id")]
    pub location_id: Option<u64>,
    pub geoname: Option<AlbumTagGeoname>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AlbumTagGeoname {
    pub id: u64,
    pub name: String,
    #[serde(rename = "fullname")]
    pub full_name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AlbumTrack {
    #[serde(rename = "track_id")]
    pub id: u64,
    pub title: String,
    #[serde(rename = "track_num")]
    pub track_number: u64,
    /// Duration is undefined for unstreamable tracks or unreleased tracks
    pub duration: Option<f32>,
    pub streaming_url: HashMap<String, String>,
    pub band_id: u64,
    pub band_name: String,
    pub label: Option<String>,
    pub label_id: Option<u64>,
    pub encodings_id: Option<u64>,
    pub album_id: Option<u64>,
    pub album_title: Option<String>,
    pub is_streamable: bool,
    pub has_lyrics: bool,
    #[serde(flatten)]
    pub image_id: ImageId,
    #[serde(flatten)]
    pub purchase_options: PurchaseOptions,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PurchaseOptions {
    pub is_set_price: bool,
    pub price: Option<f32>,
    pub currency: Option<String>,
    pub is_purchasable: bool,
    pub has_digital_download: bool,
    /// Require an email address for a free download, see
    /// https://get.bandcamp.help/hc/en-us/articles/23020667057943-How-do-I-collect-emails
    pub require_email: bool,
}

/// Fetch album
pub fn fetch_album(band_id: u64, album_id: u64) -> Result<Album, Error> {
    let url = format!(
        "{}?band_id={band_id}&tralbum_id={album_id}&tralbum_type=a",
        ALBUMS_URL
    );
    let (content, _) = get_url(url)?;
    serde_json::from_str(&content).context(SerdeSnafu)
}

/// Fetch track
pub fn fetch_track(band_id: u64, album_id: u64) -> Result<Album, Error> {
    let url = format!(
        "{}?band_id={band_id}&tralbum_id={album_id}&tralbum_type=t",
        ALBUMS_URL
    );
    let (content, _) = get_url(url)?;
    let mut file = std::fs::File::create("result.json").unwrap();
    file.write_all(content.as_bytes()).unwrap();
    serde_json::from_str(&content).context(SerdeSnafu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meschera() {
        let result = fetch_album(3752216131, 83593492).unwrap();
        assert_eq!(result.title, "Legends of the Ancients".to_string());
    }

    #[test]
    fn test_track() {
        let result = fetch_track(3752216131, 3279776665).unwrap();
        assert_eq!(result.title, "Children of the Ancient Forests".to_string());
    }

    #[test]
    fn test_geo_location() {
        fetch_track(989192576, 1585846251).unwrap();
    }
}

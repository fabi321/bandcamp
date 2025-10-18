use regex::{Captures, Regex};

mod album;
mod artist;
pub(crate) mod date_serializer;
mod error;
mod search;
mod util;

pub use album::{
    Album, AlbumBand, AlbumTag, AlbumTagGeoname, AlbumTrack, AlbumType, PurchaseOptions,
    fetch_album, fetch_track,
};
pub use artist::{
    Artist, ArtistDiscographyEntry, ArtistDiscographyEntryType, ArtistSite, LabelArtist,
    fetch_artist,
};
pub use error::Error;
use lazy_static::lazy_static;
pub use search::{
    BandcampUrl, SearchResultItem, SearchResultItemAlbum, SearchResultItemArtist,
    SearchResultItemFan, SearchResultItemTrack, search,
};
pub use util::{AlbumImage, Image, ImageResolution};

lazy_static! {
    static ref BAND_ID: Regex =
        Regex::new(r#"(?:"name":"band_id","value":|&amp;band_id=)([0-9]+)"#)
            .expect("invalid regex");
    static ref ITEM_ID: Regex =
        Regex::new(r#"(?:"name":"item_id","value":|&amp;item_id=)([0-9]+)"#)
            .expect("invalid regex");
}

fn parse_captures(captures: &Captures) -> u64 {
    captures
        .get(1)
        .expect("Unable to get capture")
        .as_str()
        .parse::<u64>()
        .expect("Unable to parse capture count")
}

pub async fn artist_from_url(url: &str) -> Result<Artist, Error> {
    let (page, _) = util::get_url(url.to_string()).await?;
    let Some(band_id) = BAND_ID.captures(&page) else {
        return Err(Error::NotFoundError {
            url: url.to_string(),
        });
    };
    fetch_artist(parse_captures(&band_id)).await
}

pub async fn album_from_url(url: &str) -> Result<Album, Error> {
    let (page, _) = util::get_url(url.to_string()).await?;
    let Some(band_id) = BAND_ID.captures(&page) else {
        return Err(Error::NotFoundError {
            url: url.to_string(),
        });
    };
    let Some(item_id) = ITEM_ID.captures(&page) else {
        return Err(Error::NotFoundError {
            url: url.to_string(),
        });
    };
    fetch_album(parse_captures(&band_id), parse_captures(&item_id)).await
}

pub async fn track_from_url(url: &str) -> Result<Album, Error> {
    let (page, _) = util::get_url(url.to_string()).await?;
    let Some(band_id) = BAND_ID.captures(&page) else {
        return Err(Error::NotFoundError {
            url: url.to_string(),
        });
    };
    let Some(item_id) = ITEM_ID.captures(&page) else {
        return Err(Error::NotFoundError {
            url: url.to_string(),
        });
    };
    fetch_track(parse_captures(&band_id), parse_captures(&item_id)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_artist_from_url() {
        artist_from_url("https://myrkur.bandcamp.com")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_artist_from_url_other() {
        artist_from_url("https://meschera.bandcamp.com")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_album_from_url() {
        album_from_url("https://myrkur.bandcamp.com/album/spine")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_hard_album_from_url() {
        album_from_url("https://othalan.bandcamp.com/album/przed-wit")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_track_from_url() {
        track_from_url("https://myrkur.bandcamp.com/track/like-humans")
            .await
            .unwrap();
    }
}

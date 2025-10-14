use regex::Regex;

mod album;
mod artist;
pub(crate) mod date_serializer;
mod error;
mod search;
mod util;

use crate::error::InvalidUrlSnafu;
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
use snafu::OptionExt;
pub use util::{AlbumImage, Image, ImageResolution};

lazy_static! {
    static ref ARTIST_URL: Regex =
        Regex::new("^(?:https?://)?([a-z]+).bandcamp.com").expect("invalid regex");
    static ref ALBUM_URL: Regex =
        Regex::new("^(?:https?://)?([a-z]+).bandcamp.com/album/([a-z-0-9]+)")
            .expect("invalid regex");
    static ref TRACK_URL: Regex =
        Regex::new("^(?:https?://)?([a-z]+).bandcamp.com/track/([a-z-0-9]+)")
            .expect("invalid regex");
}

pub async fn artist_from_url(url: &str) -> Result<Artist, Error> {
    let caputres = ARTIST_URL.captures(url).with_context(|| InvalidUrlSnafu {
        url: url.to_string(),
    })?;
    let search_result = search(&caputres[1]).await?;
    let needle = format!("https://{}.bandcamp.com", &caputres[1]);
    for result in search_result {
        if let SearchResultItem::Artist(artist) = result {
            if artist.url.starts_with(&needle) {
                return fetch_artist(artist.artist_id).await;
            }
        }
    }
    Err(Error::NotFoundError {
        url: url.to_string(),
    })
}

pub async fn album_from_url(url: &str) -> Result<Album, Error> {
    let captures = ALBUM_URL.captures(url).with_context(|| InvalidUrlSnafu {
        url: url.to_string(),
    })?;
    let query = format!("{} {}", &captures[1], &captures[2]);
    let search_result = search(&query).await?;
    let needle = format!(
        "https://{}.bandcamp.com/album/{}",
        &captures[1], &captures[2]
    );
    for result in search_result {
        if let SearchResultItem::Album(album) = result {
            if album.url.item_url.starts_with(&needle) {
                return fetch_album(album.band_id, album.album_id).await;
            }
        }
    }
    Err(Error::NotFoundError {
        url: url.to_string(),
    })
}

pub async fn track_from_url(url: &str) -> Result<Album, Error> {
    let captures = TRACK_URL.captures(url).with_context(|| InvalidUrlSnafu {
        url: url.to_string(),
    })?;
    let query = format!("{} {}", &captures[1], &captures[2]);
    let search_result = search(&query).await?;
    let needle = format!(
        "https://{}.bandcamp.com/track/{}",
        &captures[1], &captures[2]
    );
    for result in search_result {
        if let SearchResultItem::Track(track) = result {
            if track.url.item_url.starts_with(&needle) {
                return fetch_track(track.band_id, track.track_id).await;
            }
        }
    }
    Err(Error::NotFoundError {
        url: url.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_artist_from_url() {
        artist_from_url("myrkur.bandcamp.com").await.unwrap();
    }

    #[tokio::test]
    async fn test_get_album_from_url() {
        album_from_url("myrkur.bandcamp.com/album/spine")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_track_from_url() {
        track_from_url("myrkur.bandcamp.com/track/like-humans")
            .await
            .unwrap();
    }
}

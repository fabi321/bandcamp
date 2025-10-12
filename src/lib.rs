mod album;
mod artist;
pub(crate) mod date_serializer;
mod error;
mod search;
mod util;

pub use album::{
    Album, AlbumBand, AlbumTag, AlbumTrack, AlbumType, PurchaseOptions, fetch_album, fetch_track,
};
pub use artist::{
    Artist, ArtistDiscographyEntry, ArtistDiscographyEntryType, ArtistSite, LabelArtist,
    fetch_artist,
};
pub use error::Error;
pub use search::{
    BandcampUrl, ImageId, SearchResultItem, SearchResultItemAlbum, SearchResultItemArtist,
    SearchResultItemFan, SearchResultItemTrack, search,
};

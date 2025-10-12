use ::bandcamp as core_lib;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(bandcamp, BandcampError, pyo3::exceptions::PyException);

fn map_error<T>(result: Result<T, core_lib::Error>) -> PyResult<T> {
    match result {
        Ok(inner) => Ok(inner),
        Err(error) => Err(BandcampError::new_err(error.to_string())),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod bandcamp {
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::BandcampError;
    #[pymodule_export]
    use ::bandcamp::{
        Album, AlbumBand, AlbumTag, AlbumTagGeoname, AlbumTrack, AlbumType, Artist,
        ArtistDiscographyEntry, ArtistDiscographyEntryType, ArtistSite, BandcampUrl, ImageId,
        LabelArtist, PurchaseOptions, SearchResultItem, SearchResultItemAlbum,
        SearchResultItemArtist, SearchResultItemFan, SearchResultItemTrack,
    };
    use super::map_error;
    use ::bandcamp as core_lib;

    #[pyfunction]
    fn fetch_artist(artist_id: u64) -> PyResult<Artist> {
        map_error(core_lib::fetch_artist(artist_id))
    }

    #[pyfunction]
    fn fetch_album(artist_id: u64, album_id: u64) -> PyResult<Album> {
        map_error(core_lib::fetch_album(artist_id, album_id))
    }

    #[pyfunction]
    fn fetch_track(artist_id: u64, track_id: u64) -> PyResult<Album> {
        map_error(core_lib::fetch_track(artist_id, track_id))
    }

    #[pyfunction]
    fn album_from_url(url: String) -> PyResult<Album> {
        map_error(core_lib::album_from_url(&url))
    }

    #[pyfunction]
    fn artist_from_url(url: String) -> PyResult<Artist> {
        map_error(core_lib::artist_from_url(&url))
    }

    #[pyfunction]
    fn track_from_url(url: String) -> PyResult<Album> {
        map_error(core_lib::track_from_url(&url))
    }

    #[pyfunction]
    fn search(query: String, py: Python<'_>) -> PyResult<Vec<Py<PyAny>>> {
        let results = map_error(core_lib::search(&query))?;
        let mut mapped_results = Vec::new();
        for item in results {
            mapped_results.push(match item {
                SearchResultItem::Artist(artist) => {artist.into_py_any(py)}
                SearchResultItem::Album(album) => {album.into_py_any(py)}
                SearchResultItem::Track(track) => {track.into_py_any(py)}
                SearchResultItem::Fan(fan) => {fan.into_py_any(py)}
            }?)
        }
        Ok(mapped_results)
    }
}

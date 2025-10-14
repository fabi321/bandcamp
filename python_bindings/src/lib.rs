use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(bandcamp_lib, BandcampError, pyo3::exceptions::PyException);

fn map_error<T>(result: Result<T, bandcamp::Error>) -> PyResult<T> {
    match result {
        Ok(inner) => Ok(inner),
        Err(error) => Err(BandcampError::new_err(error.to_string())),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod bandcamp_lib {
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::BandcampError;
    use super::map_error;
    #[pymodule_export]
    use bandcamp::{
        Album, AlbumBand, AlbumImage, AlbumTag, AlbumTagGeoname, AlbumTrack, AlbumType, Artist,
        ArtistDiscographyEntry, ArtistDiscographyEntryType, ArtistSite, BandcampUrl, Image,
        ImageResolution, LabelArtist, PurchaseOptions, SearchResultItemAlbum,
        SearchResultItemArtist, SearchResultItemFan, SearchResultItemTrack,
    };

    #[pyfunction]
    fn fetch_artist(artist_id: u64, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_error(bandcamp::fetch_artist(artist_id).await)
        })
    }

    #[pyfunction]
    fn fetch_album(artist_id: u64, album_id: u64, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_error(bandcamp::fetch_album(artist_id, album_id).await)
        })
    }

    #[pyfunction]
    fn fetch_track(artist_id: u64, track_id: u64, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_error(bandcamp::fetch_track(artist_id, track_id).await)
        })
    }

    #[pyfunction]
    fn album_from_url(url: String, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_error(bandcamp::album_from_url(&url).await)
        })
    }

    #[pyfunction]
    fn artist_from_url(url: String, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_error(bandcamp::artist_from_url(&url).await)
        })
    }

    #[pyfunction]
    fn track_from_url(url: String, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_error(bandcamp::track_from_url(&url).await)
        })
    }

    #[pyfunction]
    fn search(query: String, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let results = map_error(bandcamp::search(&query).await)?;
            Python::attach(|py| {
                let mut mapped_results = Vec::new();
                for item in results {
                    mapped_results.push(match item {
                        bandcamp::SearchResultItem::Artist(artist) => artist.into_py_any(py),
                        bandcamp::SearchResultItem::Album(album) => album.into_py_any(py),
                        bandcamp::SearchResultItem::Track(track) => track.into_py_any(py),
                        bandcamp::SearchResultItem::Fan(fan) => fan.into_py_any(py),
                    }?)
                }
                Ok(mapped_results)
            })
        })
    }
}

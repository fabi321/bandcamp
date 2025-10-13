use snafu::prelude::*;
use std::string::FromUtf8Error;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not fetch page {url}: {source}"))]
    #[snafu(visibility(pub(crate)))]
    RequestError { source: curl::Error, url: String },
    #[snafu(display("Could not parse response: {source}"))]
    #[snafu(visibility(pub(crate)))]
    SerdeError { source: serde_json::Error },
    #[snafu(display("Could not decode response fdr {url}: {source}"))]
    #[snafu(visibility(pub(crate)))]
    ResponseDecodeError { source: FromUtf8Error, url: String },
    #[snafu(display("Could not find Artist/track/album with url: {url}"))]
    #[snafu(visibility(pub(crate)))]
    NotFoundError { url: String },
    #[snafu(display("Invalid Artist/track/album url: {url}"))]
    #[snafu(visibility(pub(crate)))]
    InvalidUrlError { url: String },
}

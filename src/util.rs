use crate::error::{Error, RequestSnafu, ResponseDecodeSnafu};
use curl::easy::Easy;
use snafu::ResultExt;

fn inner_get(url: &str) -> Result<(Vec<u8>, Option<String>), curl::Error> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.follow_location(true)?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let url = easy.effective_url()?.map(|s| s.to_string());

    Ok((data, url))
}
pub(crate) fn get_url(url: String) -> Result<(String, Option<String>), Error> {
    let (content, actual_url) =
        inner_get(&url).with_context(|_| RequestSnafu { url: url.clone() })?;
    let result = String::from_utf8(content).with_context(|_| ResponseDecodeSnafu { url })?;
    Ok((result, actual_url))
}

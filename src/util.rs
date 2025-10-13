use crate::error::{Error, RequestSnafu, ResponseDecodeSnafu};
use curl::easy::Easy;
use serde::{Deserialize, Deserializer};
use snafu::ResultExt;

fn inner_get(url: &str) -> Result<(Vec<u8>, Option<String>), curl::Error> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.follow_location(true)?;
    easy.fail_on_error(true)?;

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

pub(crate) fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

macro_rules! create_image_type {
    ($name:ident, $url_extra:literal) => {
        #[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
        #[cfg_attr(feature = "pyo3", pyo3::pyclass(eq))]
        pub struct $name {
            #[serde(default)]
            image_id: Option<u64>,
            #[serde(default)]
            img_id: Option<u64>,
            #[serde(default)]
            art_id: Option<u64>,
            #[serde(default)]
            bio_image_id: Option<u64>,
        }

        #[cfg_attr(feature = "pyo3", pyo3::pymethods)]
        impl $name {
            pub fn get_image_id(&self) -> Option<u64> {
                self.image_id
                    .or(self.img_id.or(self.art_id.or(self.bio_image_id)))
            }

            pub fn get_url(&self) -> Option<String> {
                self.get_image_id()
                    .map(|id| format!("https://f4.bcbits.com/img/{}{:010}_0.jpg", $url_extra, id))
            }
        }
    };
}

create_image_type!(AlbumImage, "a");
create_image_type!(Image, "");

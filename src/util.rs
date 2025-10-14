use crate::error::{Error, RequestSnafu};
use serde::{Deserialize, Deserializer};
use snafu::ResultExt;

async fn inner_get(url: &str) -> Result<(String, Option<String>), reqwest::Error> {
    let client = reqwest::Client::builder().use_rustls_tls().build()?;
    let response = client
        .get(url)
        .header("User-Agent", "curl/8.5.0")
        .header("Accept", "*/*")
        .send()
        .await?;

    response.error_for_status_ref()?;
    let data = response.text().await?;

    Ok((data, None))
}
pub(crate) async fn get_url(url: String) -> Result<(String, Option<String>), Error> {
    let (content, actual_url) = inner_get(&url)
        .await
        .with_context(|_| RequestSnafu { url: url.clone() })?;
    Ok((content, actual_url))
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

use crate::error::{Error, RequestSnafu};
use reqwest::{Client, header};
use serde::{Deserialize, Deserializer};
use snafu::ResultExt;

async fn inner_get(url: &str) -> Result<(String, Option<String>), reqwest::Error> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .expect("Unable to build client");
    let response = client
        .get(url)
        .header(header::USER_AGENT, "curl/8.5.0")
        .header(header::ACCEPT, "*/*")
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, eq, eq_int))]
#[cfg_attr(test, derive(strum_macros::EnumIter, strum_macros::AsRefStr))]
pub enum ImageResolution {
    Full = 0,
    Px25 = 22,
    Px50 = 42,
    Px60x45 = 140,
    Px40x80 = 161,
    Px90 = 101,
    Px60x100 = 160,
    Px100 = 3,
    Px100x75 = 29,
    Px120 = 21,
    Px124 = 8,
    Px135 = 15,
    Px138 = 12,
    Px140 = 50,
    Px144x108 = 38,
    Px150 = 7,
    Px168x126 = 37,
    Px172 = 11,
    Px180 = 206,
    Px200 = 44,
    Px210 = 9,
    /// Image with "Gift" in the top right hand corner
    Px210Gift = 165,
    Px240 = 205,
    Px270 = 70,
    Px280 = 201,
    Px300 = 4,
    Px350 = 2,
    /// Grayscale 350x350px image
    Px350Grayscale = 300,
    Px360 = 204,
    Px368 = 14,
    Px368x276 = 33,
    Px380 = 13,
    Px380x285 = 32,
    Px400x300 = 36,
    Px420 = 200,
    Px422 = 170,
    Px540 = 71,
    Px640x124 = 120,
    Px646 = 171,
    Px700 = 5,
    Px715x402 = 27,
    Px768x432 = 28,
    Px800x600 = 26,
    Px900x468 = 220,
    Px975x180PNG = 100,
    Px1024 = 20,
    Px1024PNG = 31,
    Px1200 = 10,
    Px1280x720 = 150,
    Px720x1280 = 151,
    Px3000PNG = 1,
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
                self.get_with_resolution(ImageResolution::Full)
            }

            pub fn get_with_resolution(&self, resolution: ImageResolution) -> Option<String> {
                self.get_image_id().map(|id| {
                    format!(
                        "https://f4.bcbits.com/img/{}{:010}_{}.jpg",
                        $url_extra, id, resolution as isize
                    )
                })
            }
        }
    };
}

create_image_type!(AlbumImage, "a");
create_image_type!(Image, "");

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, ImageFormat, ImageReader};
    use regex::Regex;
    use std::io::Cursor;
    use strum::IntoEnumIterator;

    impl ImageResolution {
        fn dimensions(&self) -> (u32, u32) {
            if let ImageResolution::Full = self {
                return (3000, 3000);
            }

            // regex captures the width and optional height
            // e.g. "Px60x45" -> caps[1] = "60", caps[2] = Some("45")
            let re = Regex::new(r"^Px(\d+)(?:x(\d+))?").unwrap();
            let name = self.as_ref();

            let caps = re.captures(name).expect("Failed to match name");
            let width: u32 = caps[1].parse().unwrap();
            let height: u32 = caps.get(2).map_or(width, |m| m.as_str().parse().unwrap());
            (width, height)
        }
    }

    #[tokio::test]
    async fn test_image_resolutions() {
        let image = AlbumImage {
            image_id: Some(601049957),
            img_id: None,
            bio_image_id: None,
            art_id: None,
        };
        let client = Client::builder()
            .use_rustls_tls()
            .build()
            .expect("Unable to build client");

        for format in ImageResolution::iter() {
            println!("Testing {:?}", format);

            let url = image.get_with_resolution(format).unwrap();
            let request = client
                .get(&url)
                .header(header::USER_AGENT, "curl/8.5.0")
                .header(header::ACCEPT, "*/*")
                .send()
                .await
                .expect(format!("Failed to request {url:?}").as_str());
            request
                .error_for_status_ref()
                .expect(format!("Got response {:?}", request.status()).as_str());

            let content = request.bytes().await.expect("Failed to receive iamge");
            assert!(content.len() > 0, "content of {url:?} is empty");

            let mut reader = ImageReader::new(Cursor::new(content));
            reader.set_format(if format.as_ref().contains("PNG") {
                ImageFormat::Png
            } else {
                ImageFormat::Jpeg
            });
            let loaded_image = reader.decode().expect("Failed to decode image");

            assert_eq!(format.dimensions(), loaded_image.dimensions());
        }
    }
}

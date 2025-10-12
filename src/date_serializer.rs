use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer};

const FORMAT: &'static str = "%d %b %Y %H:%M:%S %Z";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}

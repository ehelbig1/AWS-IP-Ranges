use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpRanges {
    pub sync_token: String,

    #[serde(with = "timestamp")]
    pub create_date: DateTime<Utc>,

    pub prefixes: Vec<Prefix>
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Prefix {
    pub ip_prefix: String,
    pub region: String,
    pub service: String,
    pub network_border_group: String
}

mod timestamp {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d-%H-%M-%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
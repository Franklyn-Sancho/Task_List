use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(serde::de::Error::custom("Invalid date format")),
    }
}

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    match NaiveTime::parse_from_str(&time_str, "%H:%M:%S") {
        Ok(time) => Ok(time),
        Err(_) => Err(serde::de::Error::custom("Invalid time format")),
    }
}

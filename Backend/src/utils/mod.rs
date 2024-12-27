// Modules
mod error;

pub use self::error::{Error, Result};

use chrono::prelude::*;
use chrono::TimeDelta;

// Datetime Utils
// Formats a UTC datetime to a string
pub fn format_time(time: DateTime<Utc>) -> String {
	time.to_rfc3339()
}

// Adds a number of seconds to a now UTC datetime
pub fn now_add_sec(sec: i64) -> DateTime<Utc> {
	Utc::now() + TimeDelta::try_seconds(sec).expect("i64::MAX/1000 or less than -i64::MAX/000")
}

// Parses a string to a UTC datetime
pub fn parse_utc_from_str(moment: &str) -> Result<DateTime<Utc>> {
    moment.parse::<DateTime<Utc>>().map_err(|_| Error::DateFailParse(moment.to_string()))
}

// Parses a timestamp to a UTC datetime
pub fn parse_utc_from_timestamp(timestamp: i64) -> Result<DateTime<Utc>> {
    DateTime::from_timestamp(timestamp, 0).ok_or(Error::DateFailParse(timestamp.to_string()))
}

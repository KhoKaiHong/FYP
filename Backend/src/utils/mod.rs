// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use chrono::prelude::*;
use chrono::TimeDelta;

// endregion: --- Modules

// region:    --- Time
pub fn now_utc() -> DateTime<Utc> {
	Utc::now()
}

pub fn format_time(time: DateTime<Utc>) -> String {
	time.to_rfc3339()
}

pub fn now_add_sec(sec: i64) -> Result<DateTime<Utc>> {
	let duration = TimeDelta::try_seconds(sec).ok_or(Error::InvalidSecondFormat(sec.to_string()))?;

    Ok(now_utc() + duration)
}

pub fn parse_utc_from_str(moment: &str) -> Result<DateTime<Utc>> {
    moment.parse::<DateTime<Utc>>().map_err(|_| Error::DateFailParse(moment.to_string()))
}

pub fn parse_utc_from_timestamp(timestamp: i64) -> Result<DateTime<Utc>> {
    DateTime::from_timestamp(timestamp, 0).ok_or(Error::DateFailParse(timestamp.to_string()))
}

pub fn parse_timestamp_from_utc(time: DateTime<Utc>) -> i64 {
	time.timestamp()
}
// endregion: --- Time

// region:    --- Base64
pub fn b64u_encode(content: &str) -> String {
	base64_url::encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<String> {
	let decoded_string = base64_url::decode(b64u)
		.ok()
		.and_then(|r| String::from_utf8(r).ok())
		.ok_or(Error::FailToB64uDecode)?;

	Ok(decoded_string)
}
// endregion: --- Base64

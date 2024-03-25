use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Saver's History Interval Scheme ***

{
		"endTime": "1710201600",
		"saversCount": "1927",
		"saversDepth": "82039235328",
		"saversUnits": "78972192040",
		"startTime": "1710115200"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SaversHistoryInterval {
	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "saversCount", deserialize_with = "deserialize_number_from_string")]
	savers_count: u64,

	#[serde(rename = "saversDepth", deserialize_with = "deserialize_number_from_string")]
	savers_depth: u64,

	#[serde(rename = "saversUnits", deserialize_with = "deserialize_number_from_string")]
	savers_units: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,
}

impl SaversHistoryInterval {
	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_savers_count(&self) -> &u64 {
		&self.savers_count
	}

	#[must_use]
	pub const fn get_savers_depth(&self) -> &u64 {
		&self.savers_depth
	}

	#[must_use]
	pub const fn get_savers_units(&self) -> &u64 {
		&self.savers_units
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}
}

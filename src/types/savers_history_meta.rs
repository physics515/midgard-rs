use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Saver's History Meta Scheme ***

{
		"endSaversCount": "1927",
		"endSaversDepth": "82039235328",
		"endTime": "1710201600",
		"endUnits": "78972192040",
		"startSaversCount": "1968",
		"startSaversDepth": "88435616777",
		"startTime": "1707609600",
		"startUnits": "85256259009"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SaversHistoryMeta {
	#[serde(rename = "endSaversCount", deserialize_with = "deserialize_number_from_string")]
	end_savers_count: u64,

	#[serde(rename = "endSaversDepth", deserialize_with = "deserialize_number_from_string")]
	end_savers_depth: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "endUnits", deserialize_with = "deserialize_number_from_string")]
	end_units: u64,

	#[serde(rename = "startSaversCount", deserialize_with = "deserialize_number_from_string")]
	start_savers_count: u64,

	#[serde(rename = "startSaversDepth", deserialize_with = "deserialize_number_from_string")]
	start_savers_depth: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,

	#[serde(rename = "startUnits", deserialize_with = "deserialize_number_from_string")]
	start_units: u64,
}

impl SaversHistoryMeta {
	#[must_use]
	pub const fn get_end_savers_count(&self) -> &u64 {
		&self.end_savers_count
	}

	#[must_use]
	pub const fn get_end_savers_depth(&self) -> &u64 {
		&self.end_savers_depth
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_end_units(&self) -> &u64 {
		&self.end_units
	}

	#[must_use]
	pub const fn get_start_savers_count(&self) -> &u64 {
		&self.start_savers_count
	}

	#[must_use]
	pub const fn get_start_savers_depth(&self) -> &u64 {
		&self.start_savers_depth
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}

	#[must_use]
	pub const fn get_start_units(&self) -> &u64 {
		&self.start_units
	}
}

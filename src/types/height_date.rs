use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampNanoSeconds;

/*
*** HeightDate Scheme ***

{
		"height": "2000000",
		"date": "946684801000000000"
}
*/

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightDate {
	#[serde(deserialize_with = "deserialize_number_from_string")]
	height: u64,

	#[serde_as(as = "TimestampNanoSeconds<String, Flexible>")]
	date: DateTime<Utc>,
}

impl HeightDate {
	#[must_use]
	pub const fn get_height(&self) -> u64 {
		self.height
	}

	#[must_use]
	pub const fn get_date(&self) -> &DateTime<Utc> {
		&self.date
	}
}

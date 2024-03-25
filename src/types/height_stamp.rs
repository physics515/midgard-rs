use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*
*** HeightStamp Scheme ***

{
		"height": 0,
		"timestamp": 0
}
*/

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightStamp {
	height: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	timestamp: DateTime<Utc>,
}

impl HeightStamp {
	#[must_use]
	pub const fn get_height(&self) -> u64 {
		self.height
	}

	#[must_use]
	pub const fn get_timestamp(&self) -> DateTime<Utc> {
		self.timestamp
	}
}

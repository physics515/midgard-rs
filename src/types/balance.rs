use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampNanoSeconds;

use crate::AssetAmounts;

/*

*** Balance Scheme ***
{
	"coins": AssetAmounts,
	"date": "1710779252869332524",
	"height": "15166269"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Balance {
	coins: AssetAmounts,

	#[serde_as(as = "TimestampNanoSeconds<String, Flexible>")]
	date: DateTime<Utc>,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	height: u64,
}

impl Balance {
	#[must_use]
	pub const fn get_coins(&self) -> &AssetAmounts {
		&self.coins
	}

	#[must_use]
	pub const fn get_date(&self) -> &DateTime<Utc> {
		&self.date
	}

	#[must_use]
	pub const fn get_height(&self) -> &u64 {
		&self.height
	}
}

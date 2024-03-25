use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

use crate::TVLPoolDepths;

/*

*** TVL Interval Scheme ***
{
		"endTime": "1707782400",
		"poolsDepth": TVLPoolDepths,
		"runePriceUSD": "5.417852169697646",
		"startTime": "1707696000",
		"totalValuePooled": "6230125879959270"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TVLInterval {
	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "poolsDepth")]
	pools_depth: TVLPoolDepths,

	#[serde(rename = "runePriceUSD", with = "rust_decimal::serde::str")]
	rune_price_usd: Decimal,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,

	#[serde(rename = "totalValuePooled", deserialize_with = "deserialize_number_from_string")]
	total_value_pooled: u64,
}

impl TVLInterval {
	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_pools_depth(&self) -> &TVLPoolDepths {
		&self.pools_depth
	}

	#[must_use]
	pub const fn get_rune_price_usd(&self) -> &Decimal {
		&self.rune_price_usd
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}

	#[must_use]
	pub const fn get_total_value_pooled(&self) -> &u64 {
		&self.total_value_pooled
	}
}

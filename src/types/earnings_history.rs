use serde::{Deserialize, Serialize};

use crate::EarningsIntervals;
use crate::EarningsMeta;

/*
*** Earnings History Scheme ***
{
		"intervals": EarningsIntervals,
		"meta": EarningsMeta
}
*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EarningsHistory {
	intervals: EarningsIntervals,
	meta: EarningsMeta,
}

impl EarningsHistory {
	#[must_use]
	pub const fn get_intervals(&self) -> &EarningsIntervals {
		&self.intervals
	}

	#[must_use]
	pub const fn get_meta(&self) -> &EarningsMeta {
		&self.meta
	}
}

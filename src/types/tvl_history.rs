use serde::{Deserialize, Serialize};

use crate::{TVLIntervals, TVLMeta};

/*

*** TVL History Scheme ***
{
		"intervals": TVLIntervals,
		"meta": TVLMeta
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TVLHistory {
	intervals: TVLIntervals,
	meta: TVLMeta,
}

impl TVLHistory {
	#[must_use]
	pub const fn get_intervals(&self) -> &TVLIntervals {
		&self.intervals
	}

	#[must_use]
	pub const fn get_meta(&self) -> &TVLMeta {
		&self.meta
	}
}

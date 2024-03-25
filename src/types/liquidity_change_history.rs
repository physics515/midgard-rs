use serde::{Deserialize, Serialize};

use crate::LiquidityChangeInterval;
use crate::LiquidityChangeIntervals;
use crate::LiquidityChangeMeta;

/*

*** Saver's History Scheme ***

{
				"intervals": LiquidityChangeIntervals,
				"meta": LiquidityChangeMeta
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiquidityChangeHistory {
	intervals: LiquidityChangeIntervals,
	meta: LiquidityChangeMeta,
}

impl LiquidityChangeHistory {
	#[must_use]
	pub const fn get_intervals(&self) -> &LiquidityChangeIntervals {
		&self.intervals
	}

	#[must_use]
	pub const fn get_meta(&self) -> &LiquidityChangeMeta {
		&self.meta
	}
}

impl IntoIterator for LiquidityChangeHistory {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = LiquidityChangeInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.intervals.into_iter()
	}
}

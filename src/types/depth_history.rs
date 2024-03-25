use serde::{Deserialize, Serialize};

use crate::DepthHistoryInterval;
use crate::DepthHistoryIntervals;
use crate::DepthHistoryMeta;

/*

*** Depth Hisory Scheme ***

{
		"intervals": DepthHistoryIntervals,
		"meta": DepthHistoryMeta
}
*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DepthHistory {
	intervals: DepthHistoryIntervals,
	meta: DepthHistoryMeta,
}

impl DepthHistory {
	#[must_use]
	pub const fn get_intervals(&self) -> &DepthHistoryIntervals {
		&self.intervals
	}

	#[must_use]
	pub const fn get_meta(&self) -> &DepthHistoryMeta {
		&self.meta
	}
}

impl IntoIterator for DepthHistory {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = DepthHistoryInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.intervals.into_iter()
	}
}

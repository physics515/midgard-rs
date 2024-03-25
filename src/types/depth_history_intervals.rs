use serde::{Deserialize, Serialize};

use crate::DepthHistoryInterval;

/*

*** DepthHistoryIntervals Scheme ***
[DepthHistoryInterval, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DepthHistoryIntervals(pub Vec<DepthHistoryInterval>);

impl DepthHistoryIntervals {
	#[must_use]
	pub const fn get_intervals(&self) -> &Vec<DepthHistoryInterval> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for DepthHistoryIntervals {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = DepthHistoryInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

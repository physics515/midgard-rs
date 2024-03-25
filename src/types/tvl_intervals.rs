use serde::{Deserialize, Serialize};

use crate::TVLInterval;

/*

*** TVL Intervals Scheme ***
[TVLInterval, ..]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TVLIntervals(Vec<TVLInterval>);

impl TVLIntervals {
	#[must_use]
	pub const fn get_intervals(&self) -> &Vec<TVLInterval> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for TVLIntervals {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = TVLInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

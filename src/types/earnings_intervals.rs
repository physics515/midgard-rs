use serde::{Deserialize, Serialize};

use crate::EarningsInterval;

/*

*** EarningsIntervals Scheme ***
[EarningsInterval, ..]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EarningsIntervals(Vec<EarningsInterval>);

impl EarningsIntervals {
	#[must_use]
	pub const fn get_intervals(&self) -> &Vec<EarningsInterval> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for EarningsIntervals {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = EarningsInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

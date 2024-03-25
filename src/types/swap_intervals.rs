use serde::{Deserialize, Serialize};

use crate::SwapInterval;

/*

*** Swap Intervals Scheme ***
[SwapInterval, SwapInterval, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SwapIntervals(Vec<SwapInterval>);

impl SwapIntervals {
	#[must_use]
	pub const fn get_swap_intervals(&self) -> &Vec<SwapInterval> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for SwapIntervals {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = SwapInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

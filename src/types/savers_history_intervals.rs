use serde::{Deserialize, Serialize};

use crate::SaversHistoryInterval;

/*
*** Saver's History Intervals Scheme ***

[SaverHistoryInterval, SaverHistoryInterval, SaverHistoryInterval, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SaversHistoryIntervals(Vec<SaversHistoryInterval>);

impl SaversHistoryIntervals {
	#[must_use]
	pub const fn get_saver_history_intervals(&self) -> &Vec<SaversHistoryInterval> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for SaversHistoryIntervals {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = SaversHistoryInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

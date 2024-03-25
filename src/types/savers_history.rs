use serde::{Deserialize, Serialize};

use crate::SaversHistoryInterval;
use crate::SaversHistoryIntervals;
use crate::SaversHistoryMeta;

/*

*** Saver's History Scheme ***

{
		"intervals": [
				{
						"endTime": "1707696000",
						"saversCount": "1968",
						"saversDepth": "88143231954",
						"saversUnits": "84971664693",
						"startTime": "1707609600"
				}
		],
		"meta": {
				"endSaversCount": "1927",
				"endSaversDepth": "82039235328",
				"endTime": "1710201600",
				"endUnits": "78972192040",
				"startSaversCount": "1968",
				"startSaversDepth": "88435616777",
				"startTime": "1707609600",
				"startUnits": "85256259009"
		}
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SaversHistory {
	intervals: SaversHistoryIntervals,
	meta: SaversHistoryMeta,
}

impl SaversHistory {
	#[must_use]
	pub const fn get_intervals(&self) -> &SaversHistoryIntervals {
		&self.intervals
	}

	#[must_use]
	pub const fn get_meta(&self) -> &SaversHistoryMeta {
		&self.meta
	}
}

impl IntoIterator for SaversHistory {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = SaversHistoryInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.intervals.into_iter()
	}
}

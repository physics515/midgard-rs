use serde::{Deserialize, Serialize};

use crate::SwapIntervals;
use crate::SwapMeta;

/*

*** Swap History Scheme ***
{
		intervals: SwapIntervals,
		meta: SwapMeta,
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SwapHistory {
	intervals: SwapIntervals,
	meta: SwapMeta,
}

impl SwapHistory {
	#[must_use]
	pub const fn get_intervals(&self) -> &SwapIntervals {
		&self.intervals
	}

	#[must_use]
	pub const fn get_meta(&self) -> &SwapMeta {
		&self.meta
	}
}

use serde::{Deserialize, Serialize};

use crate::LiquidityChangeInterval;

/*
*** Liquidity Change Intervals Scheme ***

[LiquidityChangeInterval, LiquidityChangeInterval, LiquidityChangeInterval, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiquidityChangeIntervals(Vec<LiquidityChangeInterval>);

impl LiquidityChangeIntervals {
	#[must_use]
	pub const fn get_liquidity_change_intervals(&self) -> &Vec<LiquidityChangeInterval> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for LiquidityChangeIntervals {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = LiquidityChangeInterval;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

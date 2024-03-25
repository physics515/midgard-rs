use serde::{Deserialize, Serialize};

use crate::EarningsPool;

/*

*** Earnings Pools Scheme ***
[EarningsPool, ..]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EarningsPools(Vec<EarningsPool>);

impl EarningsPools {
	#[must_use]
	pub const fn get_pools(&self) -> &Vec<EarningsPool> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for EarningsPools {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = EarningsPool;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

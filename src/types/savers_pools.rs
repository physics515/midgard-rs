use serde::{Deserialize, Serialize};

use crate::SaversPool;

/*
*** Savers Pools Scheme ***

[SaversPool, SaversPool, SaversPool, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SaversPools(Vec<SaversPool>);

impl SaversPools {
	#[must_use]
	pub const fn get_savers_pools(&self) -> &Vec<SaversPool> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for SaversPools {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = SaversPool;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

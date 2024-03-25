use serde::{Deserialize, Serialize};

use crate::BorrowersPool;

/*
*** Borrowers Pools Scheme ***

[BorrowersPool, BorrowersPool, BorrowersPool, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BorrowersPools(Vec<BorrowersPool>);

impl BorrowersPools {
	#[must_use]
	pub const fn get_borrowers_pools(&self) -> &Vec<BorrowersPool> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for BorrowersPools {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = BorrowersPool;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

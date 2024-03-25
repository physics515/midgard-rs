use serde::{Deserialize, Serialize};

use crate::TVLPoolDepth;

/*

*** TVL Pool Depths Scheme ***
[TVLPoolDepth, ..]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TVLPoolDepths(Vec<TVLPoolDepth>);

impl TVLPoolDepths {
	#[must_use]
	pub const fn get_depths(&self) -> &Vec<TVLPoolDepth> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for TVLPoolDepths {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = TVLPoolDepth;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

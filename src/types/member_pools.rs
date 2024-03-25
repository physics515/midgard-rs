use serde::{Deserialize, Serialize};

use crate::MemberPool;

/*
*** Member Pools Scheme ***

[MemberPool, MemberPool, MemberPool, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MemberPools(Vec<MemberPool>);

impl MemberPools {
	#[must_use]
	pub const fn get_member_pools(&self) -> &Vec<MemberPool> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for MemberPools {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = MemberPool;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

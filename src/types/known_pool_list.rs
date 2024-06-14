use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::PoolStatus;

/*

{
		string: PoolStatus
}

*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KnownPoolList(HashMap<String, PoolStatus>);

impl Default for KnownPoolList {
	fn default() -> Self {
		Self::new()
	}
}

impl KnownPoolList {
	#[must_use]
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn insert(&mut self, pool: String, status: PoolStatus) {
		self.0.insert(pool, status);
	}

        #[must_use] pub const fn get(&self) -> &HashMap<String, PoolStatus> {
                &self.0
        }

        #[must_use] pub fn get_pools(&self) -> Vec<String> {
                self.0.keys().cloned().collect()
        }

	#[must_use]
	pub fn get_status(&self, pool: &str) -> Option<&PoolStatus> {
		self.0.get(pool)
	}

	pub fn remove(&mut self, pool: &str) -> Option<PoolStatus> {
		self.0.remove(pool)
	}
}

impl IntoIterator for KnownPoolList {
        type Item = (String, PoolStatus);
        type IntoIter = std::collections::hash_map::IntoIter<String, PoolStatus>;

        fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
        }
}
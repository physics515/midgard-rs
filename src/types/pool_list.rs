use std::collections::HashSet;
use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use crate::Pool;

/*

*** Pool List Scheme ***

[Pool, Pool, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolList(Vec<Pool>);

impl PoolList {
	#[must_use]
	pub fn new(pools: Vec<Pool>) -> Self {
		Self(pools)
	}

	#[must_use]
	pub const fn get_pools(&self) -> &Vec<Pool> {
		&self.0
	}

	#[must_use]
	pub fn get_assets(&self) -> Vec<String> {
		let mut assets = HashSet::new();
		for pool in self.get_pools() {
			assets.insert(pool.get_asset().to_string());
		}
		assets.into_iter().collect()
	}

	#[must_use]
	pub fn get_simplified_assets(&self) -> Vec<String> {
		let mut assets = HashSet::new();
		for pool in self.get_pools() {
			let mut a = pool.get_asset().to_string();
			a = a.to_uppercase();
			a = a.split('-').collect::<Vec<&str>>()[0].to_string();
			assets.insert(a);
		}
		assets.into_iter().collect()
	}

	#[must_use]
	pub fn get_savers_pools(&self) -> Vec<Pool> {
		self.get_pools().iter().filter(|x| {
			let savers_depth = x.get_savers_depth();
			*savers_depth != Decimal::ZERO
		}).cloned().collect()
                
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for PoolList {
	type IntoIter = std::vec::IntoIter<Pool>;
	type Item = Pool;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

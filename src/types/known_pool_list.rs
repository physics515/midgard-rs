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

	#[must_use]
	pub fn get(&self, pool: &str) -> Option<&PoolStatus> {
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

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn test_known_pool_list() {
		let mut known_pool_list = KnownPoolList::new();
		known_pool_list.insert("BNB.BNB".to_string(), PoolStatus::Available);
		known_pool_list.insert("BTC.BTC".to_string(), PoolStatus::Staged);
		known_pool_list.insert("ETH.ETH".to_string(), PoolStatus::Suspended);
		assert_eq!(known_pool_list.get("BNB.BNB"), Some(&PoolStatus::Available));
		assert_eq!(known_pool_list.get("BTC.BTC"), Some(&PoolStatus::Staged));
		assert_eq!(known_pool_list.get("ETH.ETH"), Some(&PoolStatus::Suspended));
		assert_eq!(known_pool_list.get("ETH.BNB"), None);
		known_pool_list.remove("BNB.BNB");
		assert_eq!(known_pool_list.get("BNB.BNB"), None);
		known_pool_list.remove("BTC.BTC");
		assert_eq!(known_pool_list.get("BTC.BTC"), None);
		known_pool_list.remove("ETH.ETH");
		assert_eq!(known_pool_list.get("ETH.ETH"), None);
	}

	#[test]
	fn test_known_pool_list_deserialize() {
		let known_pool_list = json!({
				"AVAX.AVAX": "available",
				"AVAX.USDC-0XB97EF9EF8734C71904D8002F8B6BC66DD9C48A6E": "available",
				"AVAX.USDT-0X9702230A8EA53601F5CD2DC00FDBC13D4DF4A8C7": "available",
				"AVAX/AVAX": "available",
				"AVAX/USDC-0XB97EF9EF8734C71904D8002F8B6BC66DD9C48A6E": "available",
				"BCH.BCH": "available",
				"BCH/BCH": "available",
				"BNB.ADA-9F4": "suspended",
				"BNB.AERGO-46B": "suspended",
				"BNB.AVA-645": "staged",
				"BNB.AWC-986": "suspended",
				"BNB.BNB": "available",
				"BNB.BTCB-1DE": "available",
				"BNB.BULL-BE4": "suspended",
				"BNB.BUSD-BD1": "available",
				"BNB.CAKE-435": "staged",
		});
		let known_pool_list: KnownPoolList = serde_json::from_value(known_pool_list).unwrap();
		assert_eq!(known_pool_list.get("AVAX.AVAX"), Some(&PoolStatus::Available));
	}
}

use serde::{Deserialize, Serialize};

use crate::AssetAmount;

/*

*** Asset Amounts Scheme ***
[AssetAmount, AssetAmount, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssetAmounts(Vec<AssetAmount>);

impl AssetAmounts {
	#[must_use]
	pub const fn get_action_metadata_network_fees(&self) -> &Vec<AssetAmount> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for AssetAmounts {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = AssetAmount;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

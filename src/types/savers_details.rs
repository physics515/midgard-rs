use serde::{Deserialize, Serialize};

use crate::SaversPools;

/*

*** Savers Details Scheme ***

{
		"pools": SaversPools,
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaversDetails {
	pools: SaversPools,
}

impl SaversDetails {
	#[must_use]
	pub const fn get_pools(&self) -> &SaversPools {
		&self.pools
	}
}

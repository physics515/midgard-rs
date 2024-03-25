use serde::{Deserialize, Serialize};

use crate::MemberPools;

/*

*** Memeber Details Scheme ***

{
	"pools": MemberPools,
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberDetails {
	pools: MemberPools,
}

impl MemberDetails {
	#[must_use]
	pub const fn get_pools(&self) -> &MemberPools {
		&self.pools
	}
}

use serde::{Deserialize, Serialize};

use crate::BorrowersPools;

/*

*** Borrowers Details Scheme ***

{
	"pools": BorrowersPools,
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowersDetails {
	pools: BorrowersPools,
}

impl BorrowersDetails {
	#[must_use]
	pub const fn get_pools(&self) -> &BorrowersPools {
		&self.pools
	}
}

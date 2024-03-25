use serde::{Deserialize, Serialize};

/*

*** Thorname Entry Scheme ***

{
		"address": "thor18w0hsdru75ug0x4uvamgjn6ghlu43mr4dcypq9",
		"chain": "THOR"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThornameEntry {
	address: String,
	chain: String,
}

impl ThornameEntry {
	#[must_use]
	pub fn get_address(&self) -> &str {
		&self.address
	}

	#[must_use]
	pub fn get_chain(&self) -> &str {
		&self.chain
	}
}

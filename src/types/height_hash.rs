use serde::{Deserialize, Serialize};

/*
*** HeightHash Scheme ***
{
		"height": 0,
		"hash": "string"
}
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightHash {
	height: u64,
	hash: String,
}

impl HeightHash {
	#[must_use]
	pub const fn new(height: u64, hash: String) -> Self {
		Self { height, hash }
	}

	#[must_use]
	pub const fn get_height(&self) -> u64 {
		self.height
	}

	#[must_use]
	pub fn get_hash(&self) -> &str {
		&self.hash
	}
}

use serde::{Deserialize, Serialize};

/*
*** Thorname Reverse Lookup Scheme ***

[String, String, String, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ThornameReverseLookup(Vec<String>);

impl ThornameReverseLookup {
	#[must_use]
	pub const fn get_thorname_reverse_lookup(&self) -> &Vec<String> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for ThornameReverseLookup {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = String;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

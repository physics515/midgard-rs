use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/*

*** Borrowers List Scheme ***

[String, String, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowersList(Vec<String>);

impl BorrowersList {
	#[must_use]
	pub fn get_borrowers(&self) -> Vec<String> {
		let mut set = HashSet::new();
		for borrower in &self.0 {
			set.insert(borrower.to_string());
		}
		set.into_iter().collect()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for BorrowersList {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = String;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

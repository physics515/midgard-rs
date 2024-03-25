use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/*

*** Member List Scheme ***

[String, String, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberList(Vec<String>);

impl MemberList {
	#[must_use]
	pub fn get_members(&self) -> Vec<String> {
		let mut set = HashSet::new();
		for member in &self.0 {
			set.insert(member.to_string());
		}
		set.into_iter().collect()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for MemberList {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = String;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

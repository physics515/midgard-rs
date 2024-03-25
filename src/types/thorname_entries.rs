use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::ThornameEntry;

/*

*** Thorname Entries Scheme ***

[ThornameEntry, ThornameEntry, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThornameEntries(Vec<ThornameEntry>);

impl ThornameEntries {
	#[must_use]
	pub fn new(entries: Vec<ThornameEntry>) -> Self {
		Self(entries)
	}

	#[must_use]
	pub const fn get_entries(&self) -> &Vec<ThornameEntry> {
		&self.0
	}

	#[must_use]
	pub fn get_chains(&self) -> Vec<String> {
		let mut chains = HashSet::new();
		for entry in &self.0 {
			chains.insert(entry.get_chain().to_string());
		}
		chains.into_iter().collect()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for ThornameEntries {
	type IntoIter = std::vec::IntoIter<ThornameEntry>;
	type Item = ThornameEntry;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

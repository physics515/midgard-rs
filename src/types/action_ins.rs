use serde::{Deserialize, Serialize};

use crate::ActionIn;

/*

*** Action Ins Scheme ***
[ActionIn, ActionIn, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionIns(Vec<ActionIn>);

impl ActionIns {
	#[must_use]
	pub const fn get_action_ins(&self) -> &Vec<ActionIn> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for ActionIns {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = ActionIn;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

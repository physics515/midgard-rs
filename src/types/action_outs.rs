use serde::{Deserialize, Serialize};

use crate::ActionOut;

/*

*** Action Outs Scheme ***
[ActionOut, ActionOut, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionOuts(Vec<ActionOut>);

impl ActionOuts {
	#[must_use]
	pub const fn get_action_outs(&self) -> &Vec<ActionOut> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for ActionOuts {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = ActionOut;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

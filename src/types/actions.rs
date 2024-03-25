use serde::{Deserialize, Serialize};

use crate::Action;

/*

*** Actions Scheme ***
[Action, Action, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Actions(Vec<Action>);

impl Actions {
	#[must_use]
	pub const fn get_actions(&self) -> &Vec<Action> {
		&self.0
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for Actions {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = Action;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

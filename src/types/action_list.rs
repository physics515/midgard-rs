use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{Actions, PageTokens};

/*

*** Action List Scheme ***
{
		"actions": Actions,
		"count": "-1",
		"meta": PageTokens,
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionList {
	actions: Actions,

	#[serde(default, deserialize_with = "deserialize_option_number_from_string")]
	count: Option<i64>,

	meta: PageTokens,
}

impl ActionList {
	#[must_use]
	pub const fn get_actions(&self) -> &Actions {
		&self.actions
	}

	#[must_use]
	pub const fn get_count(&self) -> &Option<i64> {
		&self.count
	}

	#[must_use]
	pub const fn get_meta(&self) -> &PageTokens {
		&self.meta
	}
}

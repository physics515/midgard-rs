use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::ThornameEntries;

/*

*** Savers Details Scheme ***

{
		"entries": ThornameEntries,
		"expire": "117296954",
		"owner": "thor18w0hsdru75ug0x4uvamgjn6ghlu43mr4dcypq9"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThornameDetails {
	entries: ThornameEntries,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	expire: u64,
	owner: String,
}

impl ThornameDetails {
	#[must_use]
	pub const fn get_entries(&self) -> &ThornameEntries {
		&self.entries
	}

	#[must_use]
	pub const fn get_expire(&self) -> &u64 {
		&self.expire
	}

	#[must_use]
	pub const fn get_owner(&self) -> &String {
		&self.owner
	}
}

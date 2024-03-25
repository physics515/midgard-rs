use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Page Tokens Scheme ***
{
		"nextPageToken": "151028819000000122",
		"prevPageToken": "151028819000000122"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PageTokens {
	#[serde(rename = "nextPageToken", deserialize_with = "deserialize_option_number_from_string")]
	next_page_token: Option<u64>,

	#[serde(rename = "prevPageToken", deserialize_with = "deserialize_option_number_from_string")]
	prev_page_token: Option<u64>,
}

impl PageTokens {
	#[must_use]
	pub const fn get_next_page_token(&self) -> Option<u64> {
		self.next_page_token
	}

	#[must_use]
	pub const fn get_prev_page_token(&self) -> Option<u64> {
		self.prev_page_token
	}

	pub fn set_next_page_token(&mut self, next_page: Option<u64>) {
		self.next_page_token = next_page;
	}

	pub fn set_prev_page_token(&mut self, prev_page: Option<u64>) {
		self.prev_page_token = prev_page;
	}
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetActionList {
	/// Address of sender or recipient of any in/out transaction related to the action.
	pub address: Option<Vec<String>>,
	/// ID of any in/out tx related to the action
	pub txid: Option<String>,
	/// Any asset that is part of the action (CHAIN.SYMBOL). Additionally, synth, nosynth, and norune filters can be used for swap, add/withdraw actions.
	pub asset: Vec<String>,
	/// One or more unique types of action (swap, addLiquidity, withdraw, donate, refund, switch).
	pub action_type: Option<Vec<String>>,
	/// Affiliate address of the action (swap, refund).
	pub affiliate: Option<Vec<String>>,
	/// Number of actions returned, default is 50.
	pub limit: u64,
	/// Pagination offset, default is 0.
	pub offset: Option<u64>,
	/// If this is given, the actions for the next page will be given.
	pub next_page_token: Option<u64>,
	/// If this is given, the actions older than the timestamp will be given.
	pub timestamp: Option<u64>,
	/// If this is given, the actions older than the height will be given.
	pub height: Option<u64>,
	/// If this is given, the actions for the previous page will be given.
	pub prev_page_token: Option<u64>,
	/// If this is given, the actions newer than the timestamp will be given.
	pub from_timestamp: Option<u64>,
	/// If this is given, the actions newer than the height will be given.
	pub from_height: Option<u64>,
}

impl GetActionList {
        #[must_use]
	pub fn new(asset: Vec<String>, limit: u64) -> Self {
		Self {
			address: None,
			txid: None,
			asset,
			action_type: None,
			affiliate: None,
			limit,
			offset: None,
			next_page_token: None,
			timestamp: None,
			height: None,
			prev_page_token: None,
			from_timestamp: None,
			from_height: None,
		}
	}

	pub fn set_address(&mut self, address: Vec<String>) {
		self.address = Some(address);
	}

	pub fn set_txid(&mut self, txid: String) {
		self.txid = Some(txid);
	}

	pub fn set_action_type(&mut self, action_type: Vec<String>) {
		self.action_type = Some(action_type);
	}

	pub fn set_affiliate(&mut self, affiliate: Vec<String>) {
		self.affiliate = Some(affiliate);
	}

	pub fn set_offset(&mut self, offset: u64) {
		self.offset = Some(offset);
	}

	pub fn set_next_page_token(&mut self, next_page_token: u64) {
		self.next_page_token = Some(next_page_token);
	}

	pub fn set_timestamp(&mut self, timestamp: u64) {
		self.timestamp = Some(timestamp);
	}

	pub fn set_height(&mut self, height: u64) {
		self.height = Some(height);
	}

	pub fn set_prev_page_token(&mut self, prev_page_token: u64) {
		self.prev_page_token = Some(prev_page_token);
	}

	pub fn set_from_timestamp(&mut self, from_timestamp: u64) {
		self.from_timestamp = Some(from_timestamp);
	}

	pub fn set_from_height(&mut self, from_height: u64) {
		self.from_height = Some(from_height);
	}

	pub fn set_limit(&mut self, limit: u64) {
		self.limit = limit;
	}
}

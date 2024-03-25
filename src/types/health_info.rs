use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{HeightHash, HeightStamp};

/*
*** Scheme of HealthInfo ***

{
		"database": true,
		"scannerHeight": "string",
		"inSync": true,
		"lastThorNode": {
				"height": 0,
				"timestamp": 0
		},
		"lastFetched": {
				"height": 0,
				"timestamp": 0
		},
		"lastCommitted": {
				"height": 0,
				"timestamp": 0
		},
		"lastAggregated": {
				"height": 0,
				"timestamp": 0
		},
		"genesisInfo": {
				"height": 0,
				"hash": "string"
		}
}
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthInfo {
	database: Option<bool>,

	#[serde(rename = "scannerHeight", deserialize_with = "deserialize_option_number_from_string")]
	scanner_height: Option<u64>,

	#[serde(rename = "inSync")]
	in_sync: Option<bool>,

	#[serde(rename = "lastThorNode")]
	last_thor_node: Option<HeightStamp>,

	#[serde(rename = "lastFetched")]
	last_fetched: Option<HeightStamp>,

	#[serde(rename = "lastCommitted")]
	last_committed: Option<HeightStamp>,

	#[serde(rename = "lastAggregated")]
	last_aggregated: Option<HeightStamp>,

	#[serde(rename = "genesisInfo")]
	genesis_info: Option<HeightHash>,
}

impl HealthInfo {
	#[must_use]
	pub const fn get_database(&self) -> Option<bool> {
		self.database
	}

	#[must_use]
	pub const fn get_scanner_height(&self) -> &Option<u64> {
		&self.scanner_height
	}

	#[must_use]
	pub const fn get_in_sync(&self) -> Option<bool> {
		self.in_sync
	}

	#[must_use]
	pub const fn get_last_thor_node(&self) -> &Option<HeightStamp> {
		&self.last_thor_node
	}

	#[must_use]
	pub const fn get_last_fetched(&self) -> &Option<HeightStamp> {
		&self.last_fetched
	}

	#[must_use]
	pub const fn get_last_committed(&self) -> &Option<HeightStamp> {
		&self.last_committed
	}

	#[must_use]
	pub const fn get_last_aggregated(&self) -> &Option<HeightStamp> {
		&self.last_aggregated
	}

	#[must_use]
	pub const fn get_genesis_info(&self) -> &Option<HeightHash> {
		&self.genesis_info
	}
}

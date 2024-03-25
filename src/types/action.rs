use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampNanoSeconds;

use crate::ActionIns;
use crate::ActionMetadata;
use crate::ActionOuts;
use crate::ActionType;

/*

*** Action Scheme ***

{
		"date": "1710385361850070696",
		"height": "15102881",
		"in": ActionIns,
		"metadata": ActionMetadata,
		"out": ActionOuts,
		"pools": ["BSC.BNB", "BTC.BTC", ...],
		"status": "success",
		"type": ActionType
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Action {
	#[serde_as(as = "TimestampNanoSeconds<String, Flexible>")]
	date: DateTime<Utc>,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	height: u64,

	#[serde(rename = "in")]
	action_ins: ActionIns,

	metadata: ActionMetadata,

	#[serde(rename = "out")]
	action_outs: ActionOuts,

	pools: Vec<String>,

	status: String,

	#[serde(rename = "type")]
	action_type: ActionType,
}

impl Action {
	#[must_use]
	pub const fn get_date(&self) -> &DateTime<Utc> {
		&self.date
	}

	#[must_use]
	pub const fn get_height(&self) -> &u64 {
		&self.height
	}

	#[must_use]
	pub const fn get_action_ins(&self) -> &ActionIns {
		&self.action_ins
	}

	#[must_use]
	pub const fn get_metadata(&self) -> &ActionMetadata {
		&self.metadata
	}

	#[must_use]
	pub const fn get_action_outs(&self) -> &ActionOuts {
		&self.action_outs
	}

	#[must_use]
	pub const fn get_pools(&self) -> &Vec<String> {
		&self.pools
	}

	#[must_use]
	pub const fn get_status(&self) -> &String {
		&self.status
	}

	#[must_use]
	pub const fn get_action_type(&self) -> &ActionType {
		&self.action_type
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use crate::{Action, ActionType};

	#[test]
	fn deserialize_action() {
		let json = json!(
		{
				"date": "1710527743635577563",
				"height": "15125786",
				"in": [
						{
								"address": "thor15e5ssnh5zz6ahztk9q45jz9385yyv2kjndadhc",
								"coins": [],
								"txID": "4A552F834B261805018CABA16DFDF62F27E820409DF3A77A2FD17330E9ADCB55"
						}
				],
				"metadata": {
						"withdraw": {
								"asymmetry": "0",
								"basisPoints": "10000",
								"impermanentLossProtection": "0",
								"liquidityUnits": "-594386180",
								"memo": "-:BNB.AVA-645:10000",
								"networkFees": [
										{
												"amount": "2000000",
												"asset": "THOR.RUNE"
										}
								]
						}
				},
				"out": [
						{
								"address": "thor15e5ssnh5zz6ahztk9q45jz9385yyv2kjndadhc",
								"coins": [
										{
												"amount": "1360279671",
												"asset": "THOR.RUNE"
										}
								],
								"height": "15125786",
								"txID": ""
						}
				],
				"pools": [
						"BNB.AVA-645"
				],
				"status": "success",
				"type": "withdraw"
		});
		let action: Action = serde_json::from_value(json).unwrap();
		assert_eq!(action.get_date().to_rfc3339(), "2024-03-15T18:35:43.635577563+00:00");
		assert_eq!(*action.get_height(), 15125786 as u64);
		assert_eq!(action.get_pools(), &vec!["BNB.AVA-645".to_string()]);
		assert_eq!(action.get_status(), "success");
		assert_eq!(*action.get_action_type(), ActionType::Withdraw);
	}
}

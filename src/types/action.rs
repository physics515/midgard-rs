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

	/// A real `/v2/actions` entry from 2026-09-24 whose outbound transaction has
	/// no `height`.
	///
	/// `height` is optional on the spec's `Transaction` schema — only `txID`,
	/// `address` and `coins` are required — and it is genuinely absent for any
	/// outbound that has not landed in a block yet: 31 of the 40 outbounds in a
	/// plain `/v2/actions?limit=50` had none. Modelling it as a required `u64`
	/// failed the whole actions response with "missing field `height`", and
	/// whether a test caught that depended on which pool it randomly picked.
	/// This entry also carries `type: "contract"`, which the spec does not
	/// declare but live instances emit.
	#[test]
	fn deserialize_action_with_a_heightless_outbound() {
		let json = r#"
			{
				"date": "1790280269856369901",
				"height": "27969967",
				"in": [
					{
						"address": "thor162faxtq5qmerza08zqcqz4jknh66320au2aydx",
						"coins": [],
						"txID": "B9C9145CEB81F8226637D2B472FD558C1E347627A99F87FE3EA7B4C5C27A99A7"
					}
				],
				"metadata": {
					"contract": {
						"attributes": {
							"operation": "execute",
							"path": "0,1,2"
						},
						"contractType": "wasm-calc-strategy/process",
						"msg": {
							"execute": [
								"5160286875827252999"
							]
						}
					}
				},
				"out": [
					{
						"address": "thor133cq3fauvg9dtd4k7xvt0n8m8k6w26g7qy60uj204cevjaduhgdq7ueml3",
						"coins": [],
						"txID": "B9C9145CEB81F8226637D2B472FD558C1E347627A99F87FE3EA7B4C5C27A99A7"
					}
				],
				"pools": [],
				"status": "success",
				"type": "contract"
			}
		"#;
		let action: Action = serde_json::from_str(json).unwrap();

		assert_eq!(*action.get_action_type(), ActionType::Contract);
		assert_eq!(action.get_action_ins().get_action_ins()[0].get_height(), &None);
		assert_eq!(action.get_action_outs().get_action_outs()[0].get_height(), &None);
		assert_eq!(action.get_status(), "success");
		assert_eq!(action.get_height(), &27_969_967_u64);
	}
}

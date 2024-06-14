use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/*

*** Action Type Options ***

swap
addLiquidity
withdraw
donate
refund
switch

*/

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActionType {
	#[serde(alias = "swap", alias = "SWAP")]
	Swap,

	#[serde(alias = "addLiquidity", alias = "ADDLIQUIDITY")]
	AddLiquidity,

	#[serde(alias = "withdraw", alias = "WITHDRAW")]
	Withdraw,

	#[serde(alias = "donate", alias = "DONATE")]
	Donate,

	#[serde(alias = "refund", alias = "REFUND")]
	Refund,

	#[serde(alias = "switch", alias = "SWITCH")]
	Switch,
}

impl Display for ActionType {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match *self {
			Self::Swap => write!(f, "swap"),
			Self::AddLiquidity => write!(f, "addLiquidity"),
			Self::Withdraw => write!(f, "withdraw"),
			Self::Donate => write!(f, "donate"),
			Self::Refund => write!(f, "refund"),
			Self::Switch => write!(f, "switch"),
		}
	}
}

impl FromStr for ActionType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"swap" => Ok(Self::Swap),
			"addLiquidity" => Ok(Self::AddLiquidity),
			"withdraw" => Ok(Self::Withdraw),
			"donate" => Ok(Self::Donate),
			"refund" => Ok(Self::Refund),
			"switch" => Ok(Self::Switch),
			_ => Err(()),
		}
	}
}

impl Default for ActionType {
	fn default() -> Self {
		Self::Swap
	}
}

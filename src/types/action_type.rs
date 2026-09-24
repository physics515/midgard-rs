use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The type of a Midgard action.
///
/// The Midgard v2 OpenAPI spec (2.35.0) declares `swap`, `addLiquidity`,
/// `withdraw`, `donate`, `refund`, `switch`, `thorname`, `send`,
/// `runePoolDeposit` and `runePoolWithdraw`. Deployed Midgard instances also
/// emit `trade` and `contract`, which the published spec does not list.
///
/// Because `type` is a single field of a large response, an unrecognised value
/// used to fail the entire `/v2/actions` call rather than just that one action.
/// [`ActionType::Other`] keeps the crate's guarantee that everything is typed —
/// every value THORChain is known to produce has its own variant — while making
/// the next undocumented addition a value a consumer can inspect and match on
/// instead of a hard deserialization error.
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

	/// THORName registration or renewal.
	#[serde(alias = "thorname", alias = "THORNAME")]
	Thorname,

	/// A plain transfer, declared in the spec since 2.3x.
	#[serde(alias = "send", alias = "SEND")]
	Send,

	/// RUNEPool deposit.
	#[serde(rename = "runePoolDeposit", alias = "RUNEPOOLDEPOSIT")]
	RunePoolDeposit,

	/// RUNEPool withdrawal.
	#[serde(rename = "runePoolWithdraw", alias = "RUNEPOOLWITHDRAW")]
	RunePoolWithdraw,

	/// Trade-account deposit or withdrawal. Emitted by live Midgard instances
	/// but absent from the published OpenAPI spec as of 2.35.0.
	#[serde(alias = "trade", alias = "TRADE")]
	Trade,

	/// Smart-contract interaction. Emitted by live Midgard instances but absent
	/// from the published OpenAPI spec as of 2.35.0.
	#[serde(alias = "contract", alias = "CONTRACT")]
	Contract,

	/// An action type THORChain has added that this crate does not yet name.
	///
	/// The raw value is preserved verbatim, so it round-trips through
	/// serialization and a consumer can still act on it. Seeing one is a bug
	/// report: the variant belongs in this enum.
	#[serde(untagged)]
	Other(String),
}

impl Display for ActionType {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match self {
			Self::Swap => write!(f, "swap"),
			Self::AddLiquidity => write!(f, "addLiquidity"),
			Self::Withdraw => write!(f, "withdraw"),
			Self::Donate => write!(f, "donate"),
			Self::Refund => write!(f, "refund"),
			Self::Switch => write!(f, "switch"),
			Self::Thorname => write!(f, "thorname"),
			Self::Send => write!(f, "send"),
			Self::RunePoolDeposit => write!(f, "runePoolDeposit"),
			Self::RunePoolWithdraw => write!(f, "runePoolWithdraw"),
			Self::Trade => write!(f, "trade"),
			Self::Contract => write!(f, "contract"),
			Self::Other(raw) => write!(f, "{raw}"),
		}
	}
}

impl FromStr for ActionType {
	type Err = ();

	/// Never fails: an unrecognised value becomes [`ActionType::Other`], to
	/// match how the same value deserializes. The `Err` type is kept as `()`
	/// so existing `?`/`unwrap` call sites continue to compile.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"swap" => Self::Swap,
			"addLiquidity" => Self::AddLiquidity,
			"withdraw" => Self::Withdraw,
			"donate" => Self::Donate,
			"refund" => Self::Refund,
			"switch" => Self::Switch,
			"thorname" => Self::Thorname,
			"send" => Self::Send,
			"runePoolDeposit" => Self::RunePoolDeposit,
			"runePoolWithdraw" => Self::RunePoolWithdraw,
			"trade" => Self::Trade,
			"contract" => Self::Contract,
			other => Self::Other(other.to_string()),
		})
	}
}

impl Default for ActionType {
	fn default() -> Self {
		Self::Swap
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::ActionType;

	/// Every value the Midgard v2 spec (2.35.0) declares, plus the two live
	/// Midgard emits without declaring them.
	const KNOWN: [(&str, ActionType); 12] = [("swap", ActionType::Swap), ("addLiquidity", ActionType::AddLiquidity), ("withdraw", ActionType::Withdraw), ("donate", ActionType::Donate), ("refund", ActionType::Refund), ("switch", ActionType::Switch), ("thorname", ActionType::Thorname), ("send", ActionType::Send), ("runePoolDeposit", ActionType::RunePoolDeposit), ("runePoolWithdraw", ActionType::RunePoolWithdraw), ("trade", ActionType::Trade), ("contract", ActionType::Contract)];

	#[test]
	fn deserializes_every_known_action_type() {
		for (raw, expected) in KNOWN {
			let parsed: ActionType = serde_json::from_str(&format!("\"{raw}\"")).unwrap();
			assert_eq!(parsed, expected, "deserializing {raw:?}");
		}
	}

	#[test]
	fn display_and_from_str_agree_for_every_known_action_type() {
		for (raw, expected) in KNOWN {
			assert_eq!(expected.to_string(), raw, "Display for {raw:?}");
			assert_eq!(ActionType::from_str(raw), Ok(expected), "FromStr for {raw:?}");
		}
	}

	#[test]
	fn round_trips_through_serialization() {
		for (raw, expected) in KNOWN {
			let json = serde_json::to_string(&expected).unwrap();
			assert_eq!(json, format!("\"{raw}\""), "serializing {raw:?}");
		}
	}

	/// `trade` is the value that took down the whole `/v2/actions` response
	/// before it had a variant; an unknown value must not be able to do that
	/// again.
	#[test]
	fn an_unknown_action_type_is_preserved_rather_than_fatal() {
		let parsed: ActionType = serde_json::from_str("\"someFutureThing\"").unwrap();
		assert_eq!(parsed, ActionType::Other("someFutureThing".to_string()));
		assert_eq!(parsed.to_string(), "someFutureThing");
		assert_eq!(serde_json::to_string(&parsed).unwrap(), "\"someFutureThing\"");
		assert_eq!(ActionType::from_str("someFutureThing"), Ok(parsed));
	}

	/// An unknown type must not take its surrounding object down with it.
	#[test]
	fn an_unknown_action_type_does_not_fail_the_enclosing_object() {
		#[derive(serde::Deserialize)]
		struct Wrapper {
			#[serde(rename = "type")]
			action_type: ActionType,
			status: String,
		}

		let wrapper: Wrapper = serde_json::from_str(r#"{"type":"someFutureThing","status":"success"}"#).unwrap();
		assert_eq!(wrapper.action_type, ActionType::Other("someFutureThing".to_string()));
		assert_eq!(wrapper.status, "success");
	}
}

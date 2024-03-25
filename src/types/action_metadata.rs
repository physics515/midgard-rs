use serde::{Deserialize, Serialize};

use crate::ActionMetadataAddLiquidity;
use crate::ActionMetadataRefund;
use crate::ActionMetadataSwap;
use crate::ActionMetadataWithdraw;

/*

*** Action Metadata Scheme ***

{
	"swap": Option<ActionMetadataSwap>,
	"addLiquidity": Option<ActionMetadataAddLiquidity>,
		"withdraw": Option<ActionMetadataWithdraw>,
		"refund": Option<ActionMetadataRefund>,
}
*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionMetadata {
	swap: Option<ActionMetadataSwap>,

	#[serde(rename = "addLiquidity")]
	add_liquidity: Option<ActionMetadataAddLiquidity>,

	withdraw: Option<ActionMetadataWithdraw>,

	refund: Option<ActionMetadataRefund>,
}

impl ActionMetadata {
	#[must_use]
	pub const fn get_swap(&self) -> &Option<ActionMetadataSwap> {
		&self.swap
	}

	#[must_use]
	pub const fn get_add_liquidity(&self) -> &Option<ActionMetadataAddLiquidity> {
		&self.add_liquidity
	}

	#[must_use]
	pub const fn get_withdraw(&self) -> &Option<ActionMetadataWithdraw> {
		&self.withdraw
	}

	#[must_use]
	pub const fn get_refund(&self) -> &Option<ActionMetadataRefund> {
		&self.refund
	}
}

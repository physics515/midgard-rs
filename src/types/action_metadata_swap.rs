use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::ActionMetadataSwapStreamingSwapMeta;
use crate::AssetAmounts;

/*

*** Action Metadata Swap Scheme ***
{
		"affiliateAddress": "ti",
		"affiliateFee": "70",
		"isStreamingSwap": true,
		"liquidityFee": "3650",
		"memo": "=:BTC.BTC:bc1qnqpehe2jd92jk3wq4hsjqm5xt8jk6qqfm0qa4p:0/1/0:ti:70",
		"networkFees": AssetAmounts,
		"streamingSwapMeta": Option<ActionMetadataSwapStreamingSwapMeta>,
		"swapSlip": "0",
		"swapTarget": "0"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionMetadataSwap {
	#[serde(rename = "affiliateAddress")]
	affiliate_address: String,

	#[serde(rename = "affiliateFee", deserialize_with = "deserialize_number_from_string")]
	affiliate_fee: u64,

	#[serde(rename = "isStreamingSwap")]
	is_streaming_swap: bool,

	#[serde(rename = "liquidityFee", deserialize_with = "deserialize_number_from_string")]
	liquidity_fee: u64,

	memo: String,

	#[serde(rename = "networkFees")]
	network_fees: AssetAmounts,

	#[serde(rename = "streamingSwapMeta")]
	streaming_swap_meta: Option<ActionMetadataSwapStreamingSwapMeta>,

	#[serde(rename = "swapSlip", deserialize_with = "deserialize_number_from_string")]
	swap_slip: u64,

	#[serde(rename = "swapTarget", deserialize_with = "deserialize_number_from_string")]
	swap_target: u64,
}

impl ActionMetadataSwap {
	#[must_use]
	pub const fn get_affiliate_address(&self) -> &String {
		&self.affiliate_address
	}

	#[must_use]
	pub const fn get_affiliate_fee(&self) -> &u64 {
		&self.affiliate_fee
	}

	#[must_use]
	pub const fn get_is_streaming_swap(&self) -> &bool {
		&self.is_streaming_swap
	}

	#[must_use]
	pub const fn get_liquidity_fee(&self) -> &u64 {
		&self.liquidity_fee
	}

	#[must_use]
	pub const fn get_memo(&self) -> &String {
		&self.memo
	}

	#[must_use]
	pub const fn get_network_fees(&self) -> &AssetAmounts {
		&self.network_fees
	}

	#[must_use]
	pub const fn get_streaming_swap_meta(&self) -> &Option<ActionMetadataSwapStreamingSwapMeta> {
		&self.streaming_swap_meta
	}

	#[must_use]
	pub const fn get_swap_slip(&self) -> &u64 {
		&self.swap_slip
	}

	#[must_use]
	pub const fn get_swap_target(&self) -> &u64 {
		&self.swap_target
	}
}

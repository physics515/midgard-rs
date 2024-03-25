use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Swap Interval Scheme ***

{
		"averageSlip": "0.5609422343856906",
		"endTime": "1707782400",
		"runePriceUSD": "5.417852169697646",
		"startTime": "1707696000",
		"synthMintAverageSlip": "0.49173673528559003",
		"synthMintCount": "3449",
		"synthMintFees": "45112553937",
		"synthMintVolume": "265277256225952",
		"synthRedeemAverageSlip": "0.7669039145907474",
		"synthRedeemCount": "2810",
		"synthRedeemFees": "101863630733",
		"synthRedeemVolume": "302558513849561",
		"toAssetAverageSlip": "1.150107991360691",
		"toAssetCount": "926",
		"toAssetFees": "65720423962",
		"toAssetVolume": "125269292031867",
		"toRuneAverageSlip": "0.27019041365725543",
		"toRuneCount": "3046",
		"toRuneFees": "11119515286",
		"toRuneVolume": "114062146103572",
		"totalCount": "10231",
		"totalFees": "223816123918",
		"totalVolume": "807167208210952"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SwapInterval {
	#[serde(rename = "averageSlip", with = "rust_decimal::serde::str")]
	average_slip: Decimal,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "runePriceUSD", with = "rust_decimal::serde::str")]
	rune_price_usd: Decimal,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,

	#[serde(rename = "synthMintAverageSlip", with = "rust_decimal::serde::str")]
	synth_mint_average_slip: Decimal,

	#[serde(rename = "synthMintCount", deserialize_with = "deserialize_number_from_string")]
	synth_mint_count: u64,

	#[serde(rename = "synthMintFees", deserialize_with = "deserialize_number_from_string")]
	synth_mint_fees: u64,

	#[serde(rename = "synthMintVolume", deserialize_with = "deserialize_number_from_string")]
	synth_mint_volume: u64,

	#[serde(rename = "synthRedeemAverageSlip", with = "rust_decimal::serde::str")]
	synth_redeem_average_slip: Decimal,

	#[serde(rename = "synthRedeemCount", deserialize_with = "deserialize_number_from_string")]
	synth_redeem_count: u64,

	#[serde(rename = "synthRedeemFees", deserialize_with = "deserialize_number_from_string")]
	synth_redeem_fees: u64,

	#[serde(rename = "synthRedeemVolume", deserialize_with = "deserialize_number_from_string")]
	synth_redeem_volume: u64,

	#[serde(rename = "toAssetAverageSlip", with = "rust_decimal::serde::str")]
	to_asset_average_slip: Decimal,

	#[serde(rename = "toAssetCount", deserialize_with = "deserialize_number_from_string")]
	to_asset_count: u64,

	#[serde(rename = "toAssetFees", deserialize_with = "deserialize_number_from_string")]
	to_asset_fees: u64,

	#[serde(rename = "toAssetVolume", deserialize_with = "deserialize_number_from_string")]
	to_asset_volume: u64,

	#[serde(rename = "toAssetVolumeUSD", deserialize_with = "deserialize_option_number_from_string")]
	to_asset_volume_usd: Option<u64>,

	#[serde(rename = "toRuneAverageSlip", with = "rust_decimal::serde::str")]
	to_rune_average_slip: Decimal,

	#[serde(rename = "toRuneCount", deserialize_with = "deserialize_number_from_string")]
	to_rune_count: u64,

	#[serde(rename = "toRuneFees", deserialize_with = "deserialize_number_from_string")]
	to_rune_fees: u64,

	#[serde(rename = "toRuneVolume", deserialize_with = "deserialize_number_from_string")]
	to_rune_volume: u64,

	#[serde(rename = "toRuneVolumeUSD", deserialize_with = "deserialize_option_number_from_string")]
	to_rune_volume_usd: Option<u64>,

	#[serde(rename = "totalCount", deserialize_with = "deserialize_number_from_string")]
	total_count: u64,

	#[serde(rename = "totalFees", deserialize_with = "deserialize_number_from_string")]
	total_fees: u64,

	#[serde(rename = "totalVolume", deserialize_with = "deserialize_number_from_string")]
	total_volume: u64,

	#[serde(rename = "totalVolumeUSD", deserialize_with = "deserialize_option_number_from_string")]
	total_volume_usd: Option<u64>,
}

impl SwapInterval {
	#[must_use]
	pub const fn get_average_slip(&self) -> &Decimal {
		&self.average_slip
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_rune_price_usd(&self) -> &Decimal {
		&self.rune_price_usd
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}

	#[must_use]
	pub const fn get_synth_mint_average_slip(&self) -> &Decimal {
		&self.synth_mint_average_slip
	}

	#[must_use]
	pub const fn get_synth_mint_count(&self) -> &u64 {
		&self.synth_mint_count
	}

	#[must_use]
	pub const fn get_synth_mint_fees(&self) -> &u64 {
		&self.synth_mint_fees
	}

	#[must_use]
	pub const fn get_synth_mint_volume(&self) -> &u64 {
		&self.synth_mint_volume
	}

	#[must_use]
	pub const fn get_synth_redeem_average_slip(&self) -> &Decimal {
		&self.synth_redeem_average_slip
	}

	#[must_use]
	pub const fn get_synth_redeem_count(&self) -> &u64 {
		&self.synth_redeem_count
	}

	#[must_use]
	pub const fn get_synth_redeem_fees(&self) -> &u64 {
		&self.synth_redeem_fees
	}

	#[must_use]
	pub const fn get_synth_redeem_volume(&self) -> &u64 {
		&self.synth_redeem_volume
	}

	#[must_use]
	pub const fn get_to_asset_average_slip(&self) -> &Decimal {
		&self.to_asset_average_slip
	}

	#[must_use]
	pub const fn get_to_asset_count(&self) -> &u64 {
		&self.to_asset_count
	}

	#[must_use]
	pub const fn get_to_asset_fees(&self) -> &u64 {
		&self.to_asset_fees
	}

	#[must_use]
	pub const fn get_to_asset_volume(&self) -> &u64 {
		&self.to_asset_volume
	}

	#[must_use]
	pub const fn get_to_asset_volume_usd(&self) -> &Option<u64> {
		&self.to_asset_volume_usd
	}

	#[must_use]
	pub const fn get_to_rune_average_slip(&self) -> &Decimal {
		&self.to_rune_average_slip
	}

	#[must_use]
	pub const fn get_to_rune_count(&self) -> &u64 {
		&self.to_rune_count
	}

	#[must_use]
	pub const fn get_to_rune_fees(&self) -> &u64 {
		&self.to_rune_fees
	}

	#[must_use]
	pub const fn get_to_rune_volume(&self) -> &u64 {
		&self.to_rune_volume
	}

	#[must_use]
	pub const fn get_to_rune_volume_usd(&self) -> &Option<u64> {
		&self.to_rune_volume_usd
	}

	#[must_use]
	pub const fn get_total_count(&self) -> &u64 {
		&self.total_count
	}

	#[must_use]
	pub const fn get_total_fees(&self) -> &u64 {
		&self.total_fees
	}

	#[must_use]
	pub const fn get_total_volume(&self) -> &u64 {
		&self.total_volume
	}

	#[must_use]
	pub const fn get_total_volume_usd(&self) -> &Option<u64> {
		&self.total_volume_usd
	}
}

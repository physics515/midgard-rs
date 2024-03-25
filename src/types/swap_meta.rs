use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Swap Meta Scheme ***

{
		"averageSlip": "0.7043212107695739",
		"endTime": "1710288000",
		"runePriceUSD": "9.849104010393853",
		"startTime": "1707696000",
		"synthMintAverageSlip": "0.660790711470825",
		"synthMintCount": "141594",
		"synthMintFees": "2738318966140",
		"synthMintVolume": "12721267621939614",
		"synthRedeemAverageSlip": "0.7406643757159221",
		"synthRedeemCount": "126585",
		"synthRedeemFees": "4513847204931",
		"synthRedeemVolume": "12838360056854707",
		"toAssetAverageSlip": "1.1981717544448172",
		"toAssetCount": "35772",
		"toAssetFees": "4346538184030",
		"toAssetVolume": "5245279208252100",
		"toRuneAverageSlip": "0.5174002878766774",
		"toRuneCount": "86148",
		"toRuneFees": "1231526856953",
		"toRuneVolume": "5323649320824332",
		"totalCount": "390099",
		"totalFees": "12830231212054",
		"totalVolume": "36128556207870753"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SwapMeta {
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

	#[serde(rename = "synthMintVolumeUSD", deserialize_with = "deserialize_option_number_from_string")]
	synth_mint_volume_usd: Option<u64>,

	#[serde(rename = "synthRedeemAverageSlip", with = "rust_decimal::serde::str")]
	synth_redeem_average_slip: Decimal,

	#[serde(rename = "synthRedeemCount", deserialize_with = "deserialize_number_from_string")]
	synth_redeem_count: u64,

	#[serde(rename = "synthRedeemFees", deserialize_with = "deserialize_number_from_string")]
	synth_redeem_fees: u64,

	#[serde(rename = "synthRedeemVolume", deserialize_with = "deserialize_number_from_string")]
	synth_redeem_volume: u64,

	#[serde(rename = "synthRedeemVolumeUSD", deserialize_with = "deserialize_option_number_from_string")]
	synth_redeem_volume_usd: Option<u64>,

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

impl SwapMeta {
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
	pub const fn get_synth_mint_volume_usd(&self) -> &Option<u64> {
		&self.synth_mint_volume_usd
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
	pub const fn get_synth_redeem_volume_usd(&self) -> &Option<u64> {
		&self.synth_redeem_volume_usd
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

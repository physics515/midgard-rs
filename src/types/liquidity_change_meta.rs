use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Liquidity Change Meta Scheme ***

{
		"addAssetLiquidityVolume": "24500007356327",
		"addLiquidityCount": "298",
		"addLiquidityVolume": "40819031490157",
		"addRuneLiquidityVolume": "16319024133830",
		"endTime": "1710374400",
		"net": "-111119966371808",
		"runePriceUSD": "11.099857627935815",
		"startTime": "1707782400",
		"withdrawAssetVolume": "91336466174063",
		"withdrawCount": "403",
		"withdrawRuneVolume": "60602531687902",
		"withdrawVolume": "151938997861965"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiquidityChangeMeta {
	#[serde(rename = "addAssetLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_asset_liquidity_volume: u64,

	#[serde(rename = "addLiquidityCount", deserialize_with = "deserialize_number_from_string")]
	add_liquidity_count: u64,

	#[serde(rename = "addLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_liquidity_volume: u64,

	#[serde(rename = "addRuneLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_rune_liquidity_volume: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "net", deserialize_with = "deserialize_number_from_string")]
	net: i64,

	#[serde(rename = "runePriceUSD", with = "rust_decimal::serde::str")]
	rune_price_usd: Decimal,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,

	#[serde(rename = "withdrawAssetVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_asset_volume: u64,

	#[serde(rename = "withdrawCount", deserialize_with = "deserialize_number_from_string")]
	withdraw_count: u64,

	#[serde(rename = "withdrawRuneVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_rune_volume: u64,

	#[serde(rename = "withdrawVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_volume: u64,
}

impl LiquidityChangeMeta {
	#[must_use]
	pub const fn get_add_asset_liquidity_volume(&self) -> &u64 {
		&self.add_asset_liquidity_volume
	}

	#[must_use]
	pub const fn get_add_liquidity_count(&self) -> &u64 {
		&self.add_liquidity_count
	}

	#[must_use]
	pub const fn get_add_liquidity_volume(&self) -> &u64 {
		&self.add_liquidity_volume
	}

	#[must_use]
	pub const fn get_add_rune_liquidity_volume(&self) -> &u64 {
		&self.add_rune_liquidity_volume
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_net(&self) -> &i64 {
		&self.net
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
	pub const fn get_withdraw_asset_volume(&self) -> &u64 {
		&self.withdraw_asset_volume
	}

	#[must_use]
	pub const fn get_withdraw_count(&self) -> &u64 {
		&self.withdraw_count
	}

	#[must_use]
	pub const fn get_withdraw_rune_volume(&self) -> &u64 {
		&self.withdraw_rune_volume
	}

	#[must_use]
	pub const fn get_withdraw_volume(&self) -> &u64 {
		&self.withdraw_volume
	}
}

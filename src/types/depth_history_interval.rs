use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Depth History Interval ***

{
		"assetDepth": "131812998956",
		"assetPrice": "7993.962556893128",
		"assetPriceUSD": "72054.12534031394",
		"endTime": "1710172800",
		"liquidityUnits": "384907905455703",
		"luvi": "0.018671610262514394",
		"membersCount": "3640",
		"runeDepth": "1053708178166057",
		"startTime": "1710169200",
		"synthSupply": "102862463587",
		"synthUnits": "246278334048349",
		"units": "631186239504052"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepthHistoryInterval {
	#[serde(rename = "assetDepth", deserialize_with = "deserialize_number_from_string")]
	asset_depth: u64,

	#[serde(rename = "assetPrice", with = "rust_decimal::serde::str")]
	asset_price: Decimal,

	#[serde(rename = "assetPriceUSD", with = "rust_decimal::serde::str")]
	asset_price_usd: Decimal,

	/// USD price at the close of the interval. Part of the OHLC set added
	/// upstream in Midgard 2.33.0.
	#[serde(rename = "closePriceUSD", default, with = "crate::types::decimal_nan::optional")]
	close_price_usd: Option<Decimal>,

	/// Highest USD price during the interval. Added upstream in Midgard 2.33.0.
	#[serde(rename = "highPriceUSD", default, with = "crate::types::decimal_nan::optional")]
	high_price_usd: Option<Decimal>,

	/// Lowest USD price during the interval. Added upstream in Midgard 2.33.0.
	#[serde(rename = "lowPriceUSD", default, with = "crate::types::decimal_nan::optional")]
	low_price_usd: Option<Decimal>,

	/// USD price at the open of the interval. Added upstream in Midgard 2.33.0.
	#[serde(rename = "openPriceUSD", default, with = "crate::types::decimal_nan::optional")]
	open_price_usd: Option<Decimal>,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "liquidityUnits", deserialize_with = "deserialize_number_from_string")]
	liquidity_units: u64,

	/// Midgard reports `"NaN"` here whenever LUVI is undefined for the
	/// interval (a pool with no liquidity, for instance), so this is `None`
	/// rather than a misleading zero.
	#[serde(with = "crate::types::decimal_nan::optional")]
	luvi: Option<Decimal>,

	#[serde(rename = "membersCount", deserialize_with = "deserialize_number_from_string")]
	members_count: u64,

	#[serde(rename = "runeDepth", deserialize_with = "deserialize_number_from_string")]
	rune_depth: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,

	#[serde(rename = "synthSupply", deserialize_with = "deserialize_number_from_string")]
	synth_supply: u64,

	/// Signed: Midgard returns large negative synth unit counts on pools
	/// whose synths have been burnt below the recorded baseline.
	#[serde(rename = "synthUnits", deserialize_with = "deserialize_number_from_string")]
	synth_units: i64,

	/// Signed, for the same reason as [`Self::synth_units`].
	#[serde(deserialize_with = "deserialize_number_from_string")]
	units: i64,
}

impl DepthHistoryInterval {
	#[must_use]
	pub const fn get_asset_depth(&self) -> &u64 {
		&self.asset_depth
	}

	#[must_use]
	pub const fn get_asset_price(&self) -> &Decimal {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &Decimal {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_close_price_usd(&self) -> &Option<Decimal> {
		&self.close_price_usd
	}

	#[must_use]
	pub const fn get_high_price_usd(&self) -> &Option<Decimal> {
		&self.high_price_usd
	}

	#[must_use]
	pub const fn get_low_price_usd(&self) -> &Option<Decimal> {
		&self.low_price_usd
	}

	#[must_use]
	pub const fn get_open_price_usd(&self) -> &Option<Decimal> {
		&self.open_price_usd
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &u64 {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_luvi(&self) -> &Option<Decimal> {
		&self.luvi
	}

	#[must_use]
	pub const fn get_members_count(&self) -> &u64 {
		&self.members_count
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &u64 {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}

	#[must_use]
	pub const fn get_synth_supply(&self) -> &u64 {
		&self.synth_supply
	}

	#[must_use]
	pub const fn get_synth_units(&self) -> &i64 {
		&self.synth_units
	}

	#[must_use]
	pub const fn get_units(&self) -> &i64 {
		&self.units
	}
}

#[cfg(test)]
mod tests {
	use rust_decimal::Decimal;

	use super::DepthHistoryInterval;

	/// The first interval `history/depths/BTC.BTC?interval=day&count=100`
	/// returned on 2026-09-24. It carries `"NaN"` for `luvi` and negative
	/// `units`/`synthUnits`; before those fields were widened, this payload —
	/// which is the common case, not an edge case — failed to deserialize and
	/// took the entire depth-history response down with it.
	#[test]
	fn test_deserialize_interval_with_nan_luvi_and_negative_units() {
		let json = r#"{
			"assetDepth": "17327772468",
			"assetPrice": "139687.13996053414",
			"assetPriceUSD": "81803.70496820814",
			"endTime": "1781740800",
			"liquidityUnits": "233009705453414",
			"luvi": "NaN",
			"membersCount": "2998",
			"runeDepth": "2420466977941806",
			"startTime": "1781654400",
			"synthSupply": "54049458601",
			"synthUnits": "-649381483597292",
			"units": "-416371778143878"
		}"#;
		let interval: DepthHistoryInterval = serde_json::from_str(json).unwrap();

		assert_eq!(interval.get_luvi(), &None);
		assert_eq!(interval.get_units(), &-416_371_778_143_878_i64);
		assert_eq!(interval.get_synth_units(), &-649_381_483_597_292_i64);
		assert_eq!(interval.get_members_count(), &2998_u64);
	}

	/// The full interval payload as `history/depths/BTC.BTC` returns it,
	/// including the OHLC fields Midgard added in 2.33.0.
	#[test]
	fn test_deserialize_interval_with_ohlc() {
		let json = r#"{
			"assetDepth": "17327772468",
			"assetPrice": "139687.13996053414",
			"assetPriceUSD": "81803.70496820814",
			"closePriceUSD": "81900.5",
			"endTime": "1781740800",
			"highPriceUSD": "82500.25",
			"liquidityUnits": "233009705453414",
			"lowPriceUSD": "81000.75",
			"luvi": "NaN",
			"membersCount": "2998",
			"openPriceUSD": "81500",
			"runeDepth": "2420466977941806",
			"startTime": "1781654400",
			"synthSupply": "54049458601",
			"synthUnits": "-649381483597292",
			"units": "-416371778143878"
		}"#;
		let interval: DepthHistoryInterval = serde_json::from_str(json).unwrap();

		assert_eq!(interval.get_open_price_usd(), &Some("81500".parse::<Decimal>().unwrap()));
		assert_eq!(interval.get_high_price_usd(), &Some("82500.25".parse::<Decimal>().unwrap()));
		assert_eq!(interval.get_low_price_usd(), &Some("81000.75".parse::<Decimal>().unwrap()));
		assert_eq!(interval.get_close_price_usd(), &Some("81900.5".parse::<Decimal>().unwrap()));
	}

	/// An interval from before the OHLC fields existed must still parse: they
	/// are `#[serde(default)]`, so an older Midgard instance or a cached
	/// response does not become a hard error.
	#[test]
	fn test_deserialize_interval_without_ohlc() {
		let json = r#"{
			"assetDepth": "17327772468",
			"assetPrice": "139687.13996053414",
			"assetPriceUSD": "81803.70496820814",
			"endTime": "1781740800",
			"liquidityUnits": "233009705453414",
			"luvi": "1.5",
			"membersCount": "2998",
			"runeDepth": "2420466977941806",
			"startTime": "1781654400",
			"synthSupply": "54049458601",
			"synthUnits": "10",
			"units": "20"
		}"#;
		let interval: DepthHistoryInterval = serde_json::from_str(json).unwrap();

		assert_eq!(interval.get_open_price_usd(), &None);
		assert_eq!(interval.get_close_price_usd(), &None);
	}

	#[test]
	fn test_deserialize_interval_with_a_real_luvi() {
		let json = r#"{
			"assetDepth": "20789919007903",
			"assetPrice": "0.15992757",
			"assetPriceUSD": "1.0424",
			"endTime": "1710892800",
			"liquidityUnits": "901529373373",
			"luvi": "0.018671610262514394",
			"membersCount": "202",
			"runeDepth": "3324827631133",
			"startTime": "1710806400",
			"synthSupply": "1234",
			"synthUnits": "59635896754",
			"units": "961165270127"
		}"#;
		let interval: DepthHistoryInterval = serde_json::from_str(json).unwrap();

		assert_eq!(interval.get_luvi(), &Some("0.018671610262514394".parse::<Decimal>().unwrap()));
		assert_eq!(interval.get_units(), &961_165_270_127_i64);
	}
}

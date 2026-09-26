use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::PoolStatus;

/*

*** Pool Scheme ***

{
		"annualPercentageRate": "0.35621574724513394",
		"asset": "AVAX.AVAX",
		"assetDepth": "23203306126765",
		"assetPrice": "5.0534933245368965",
		"assetPriceUSD": "43.142479506616894",
		"earnings": "2973647470346",
		"earningsAnnualAsPercentOfDepth": "0.3085457187082215",
		"liquidityUnits": "79111877002320",
		"lpLuvi": "4.06224252887876",
		"nativeDecimal": "18",
		"poolAPY": "0.35621574724513394",
		"runeDepth": "117257752618793",
		"saversAPR": "0.0285071367062461",
		"saversDepth": "16855650505734",
		"saversUnits": "15522006521897",
		"status": "available",
		"synthSupply": "17233671212041",
		"synthUnits": "46734680445363",
		"totalCollateral": "0",
		"totalDebtTor": "0",
		"units": "125846557447683",
		"volume24h": "154130557380168"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pool {
	#[serde(rename = "annualPercentageRate", with = "crate::types::decimal_nan::optional")]
	annual_percentage_rate: Option<Decimal>,

	asset: String,

	#[serde(rename = "assetDepth", with = "rust_decimal::serde::str")]
	asset_depth: Decimal,

	#[serde(rename = "assetPrice", with = "crate::types::decimal_nan::optional")]
	asset_price: Option<Decimal>,

	#[serde(rename = "assetPriceUSD", with = "crate::types::decimal_nan::optional")]
	asset_price_usd: Option<Decimal>,

	/// Asset depth at which a swap would move the price 2% down. Added
	/// upstream in Midgard 2.34.0.
	#[serde(rename = "depthMinus2Percent", with = "rust_decimal::serde::str")]
	depth_minus_2_percent: Decimal,

	/// Asset depth at which a swap would move the price 2% up. Added upstream
	/// in Midgard 2.34.0.
	#[serde(rename = "depthPlus2Percent", with = "rust_decimal::serde::str")]
	depth_plus_2_percent: Decimal,

	#[serde(with = "rust_decimal::serde::str")]
	earnings: Decimal,

	/// Total pool liquidity valued in USD. Added upstream in Midgard 2.34.1.
	#[serde(rename = "liquidityInUSD", with = "rust_decimal::serde::str")]
	liquidity_in_usd: Decimal,

	#[serde(rename = "earningsAnnualAsPercentOfDepth", with = "crate::types::decimal_nan::optional")]
	earnings_annual_as_percent_of_depth: Option<Decimal>,

	#[serde(rename = "liquidityUnits", with = "rust_decimal::serde::str")]
	liquidity_units: Decimal,

	#[serde(rename = "lpLuvi", with = "crate::types::decimal_nan::optional")]
	lp_luvi: Option<Decimal>,

	#[serde(rename = "nativeDecimal", with = "rust_decimal::serde::str")]
	native_decimal: Decimal,

	#[serde(rename = "poolAPY", with = "crate::types::decimal_nan::optional")]
	pool_apy: Option<Decimal>,

	#[serde(rename = "runeDepth", with = "rust_decimal::serde::str")]
	rune_depth: Decimal,

	#[serde(rename = "saversAPR", with = "crate::types::decimal_nan::optional")]
	savers_apr: Option<Decimal>,

	#[serde(rename = "saversDepth", with = "rust_decimal::serde::str")]
	savers_depth: Decimal,

	#[serde(rename = "saversUnits", with = "rust_decimal::serde::str")]
	savers_units: Decimal,

	/// Share of pool yield paid to savers.
	///
	/// `None` in two distinct upstream cases, both observed on the live API on
	/// 2026-09-24: Midgard omits the field entirely for most pools (32 of 43),
	/// and reports the string `"NaN"` for some of the rest. Either way the
	/// share is undefined, which is not the same as a share of zero.
	#[serde(rename = "saversYieldShare", default, with = "crate::types::decimal_nan::optional")]
	savers_yield_share: Option<Decimal>,

	status: PoolStatus,

	#[serde(rename = "synthSupply", with = "rust_decimal::serde::str")]
	synth_supply: Decimal,

	#[serde(rename = "synthUnits", with = "rust_decimal::serde::str")]
	synth_units: Decimal,

	#[serde(rename = "totalCollateral", with = "rust_decimal::serde::str")]
	total_collateral: Decimal,

	#[serde(rename = "totalDebtTor", with = "rust_decimal::serde::str")]
	total_debt_tor: Decimal,

	#[serde(with = "rust_decimal::serde::str")]
	units: Decimal,

	#[serde(rename = "volume24h", with = "rust_decimal::serde::str")]
	volume_24h: Decimal,
}

impl Pool {
	#[must_use]
	pub const fn get_annual_percentage_rate(&self) -> &Option<Decimal> {
		&self.annual_percentage_rate
	}

	#[must_use]
	pub fn get_asset(&self) -> &str {
		&self.asset
	}

	#[must_use]
	pub const fn get_asset_depth(&self) -> &Decimal {
		&self.asset_depth
	}

	#[must_use]
	pub const fn get_asset_price(&self) -> &Option<Decimal> {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &Option<Decimal> {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_depth_minus_2_percent(&self) -> &Decimal {
		&self.depth_minus_2_percent
	}

	#[must_use]
	pub const fn get_depth_plus_2_percent(&self) -> &Decimal {
		&self.depth_plus_2_percent
	}

	#[must_use]
	pub const fn get_earnings(&self) -> &Decimal {
		&self.earnings
	}

	#[must_use]
	pub const fn get_liquidity_in_usd(&self) -> &Decimal {
		&self.liquidity_in_usd
	}

	#[must_use]
	pub const fn get_earnings_annual_as_percent_of_depth(&self) -> &Option<Decimal> {
		&self.earnings_annual_as_percent_of_depth
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &Decimal {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_lp_luvi(&self) -> &Option<Decimal> {
		&self.lp_luvi
	}

	#[must_use]
	pub const fn get_native_decimal(&self) -> &Decimal {
		&self.native_decimal
	}

	#[must_use]
	pub const fn get_pool_apy(&self) -> &Option<Decimal> {
		&self.pool_apy
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &Decimal {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_savers_apr(&self) -> &Option<Decimal> {
		&self.savers_apr
	}

	#[must_use]
	pub const fn get_savers_depth(&self) -> &Decimal {
		&self.savers_depth
	}

	#[must_use]
	pub const fn get_savers_units(&self) -> &Decimal {
		&self.savers_units
	}

	#[must_use]
	pub const fn get_savers_yield_share(&self) -> &Option<Decimal> {
		&self.savers_yield_share
	}

	#[must_use]
	pub const fn get_status(&self) -> &PoolStatus {
		&self.status
	}

	#[must_use]
	pub const fn get_synth_supply(&self) -> &Decimal {
		&self.synth_supply
	}

	#[must_use]
	pub const fn get_synth_units(&self) -> &Decimal {
		&self.synth_units
	}

	#[must_use]
	pub const fn get_total_collateral(&self) -> &Decimal {
		&self.total_collateral
	}

	#[must_use]
	pub const fn get_total_debt_tor(&self) -> &Decimal {
		&self.total_debt_tor
	}

	#[must_use]
	pub const fn get_units(&self) -> &Decimal {
		&self.units
	}

	#[must_use]
	pub const fn get_volume_24h(&self) -> &Decimal {
		&self.volume_24h
	}
}

#[cfg(test)]
mod tests {
	use super::Pool;

	/// A pool as `/v2/pools` returned it on 2026-09-24, including the four
	/// fields Midgard added in 2.34.0/2.34.1.
	#[test]
	fn test_deserialize_pool_with_savers_yield_share() {
		let json = r#"{
			"annualPercentageRate": "0.0039955742883688306",
			"asset": "AVAX.AVAX",
			"assetDepth": "7928002202803",
			"assetPrice": "16.462543272146103",
			"assetPriceUSD": "10.56328569664937",
			"depthMinus2Percent": "2610301586506",
			"depthPlus2Percent": "2612914501007",
			"earnings": "19963748898",
			"earningsAnnualAsPercentOfDepth": "0.003987918147870234",
			"liquidityInUSD": "1674914.0274465813",
			"liquidityUnits": "62236934161718",
			"lpLuvi": "-0.28255250841384183",
			"nativeDecimal": "18",
			"poolAPY": "0.0039955742883688306",
			"runeDepth": "130515079325314",
			"saversAPR": "0",
			"saversDepth": "3916870852770",
			"saversUnits": "3450281845752",
			"saversYieldShare": "NaN",
			"status": "available",
			"synthSupply": "4078229611474",
			"synthUnits": "21550463670958",
			"totalCollateral": "0",
			"totalDebtTor": "0",
			"units": "83787397832676",
			"volume24h": "34408696627267"
		}"#;
		let pool: Pool = serde_json::from_str(json).unwrap();

		assert_eq!(pool.get_asset(), "AVAX.AVAX");
		assert_eq!(pool.get_depth_minus_2_percent(), &"2610301586506".parse().unwrap());
		assert_eq!(pool.get_depth_plus_2_percent(), &"2612914501007".parse().unwrap());
		assert_eq!(pool.get_liquidity_in_usd(), &"1674914.0274465813".parse().unwrap());
		assert_eq!(pool.get_savers_yield_share(), &None, "this pool reports the string \"NaN\"");
	}

	/// Midgard omits `saversYieldShare` entirely for most pools — 32 of the 43
	/// live pools on 2026-09-24 — so the field must be optional as well as
	/// NaN-tolerant, or the whole `/v2/pools` response fails to parse.
	#[test]
	fn test_deserialize_pool_without_savers_yield_share() {
		let json = r#"{
			"annualPercentageRate": "0",
			"asset": "AVAX.SOL-0XFE6B19286885A4F7F55ADAD09C3CD1F906D2478F",
			"assetDepth": "1075826730",
			"assetPrice": "209.31904104390492",
			"assetPriceUSD": "134.31076813243806",
			"depthMinus2Percent": "4503820389",
			"depthPlus2Percent": "4508328717",
			"earnings": "0",
			"earningsAnnualAsPercentOfDepth": "0",
			"liquidityInUSD": "2888.7342265504",
			"liquidityUnits": "2805246443473",
			"lpLuvi": "-0.24894047036574496",
			"nativeDecimal": "9",
			"poolAPY": "0",
			"runeDepth": "225191019453",
			"saversAPR": "0",
			"saversDepth": "0",
			"saversUnits": "0",
			"status": "staged",
			"synthSupply": "137229178",
			"synthUnits": "191102573060",
			"totalCollateral": "0",
			"totalDebtTor": "0",
			"units": "2996349016533",
			"volume24h": "0"
		}"#;
		let pool: Pool = serde_json::from_str(json).unwrap();

		assert_eq!(pool.get_asset(), "AVAX.SOL-0XFE6B19286885A4F7F55ADAD09C3CD1F906D2478F");
		assert_eq!(pool.get_savers_yield_share(), &None);
		assert_eq!(pool.get_liquidity_in_usd(), &"2888.7342265504".parse().unwrap());
	}
}

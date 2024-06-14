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
	#[serde(rename = "annualPercentageRate", with = "deserialize_decimal_with_nan")]
	annual_percentage_rate: Decimal,

	asset: String,

	#[serde(rename = "assetDepth", with = "deserialize_decimal_with_nan")]
	asset_depth: Decimal,

	#[serde(rename = "assetPrice", with = "deserialize_decimal_with_nan")]
	asset_price: Decimal,

	#[serde(rename = "assetPriceUSD", with = "deserialize_decimal_with_nan")]
	asset_price_usd: Decimal,

	#[serde(with = "deserialize_decimal_with_nan")]
	earnings: Decimal,

	#[serde(rename = "earningsAnnualAsPercentOfDepth", with = "deserialize_decimal_with_nan")]
	earnings_annual_as_percent_of_depth: Decimal,

	#[serde(rename = "liquidityUnits", with = "deserialize_decimal_with_nan")]
	liquidity_units: Decimal,

	#[serde(rename = "lpLuvi", with = "deserialize_decimal_with_nan")]
	lp_luvi: Decimal,

	#[serde(rename = "nativeDecimal", with = "deserialize_decimal_with_nan")]
	native_decimal: Decimal,

	#[serde(rename = "poolAPY", with = "deserialize_decimal_with_nan")]
	pool_apy: Decimal,

	#[serde(rename = "runeDepth", with = "deserialize_decimal_with_nan")]
	rune_depth: Decimal,

	#[serde(rename = "saversAPR", with = "deserialize_decimal_with_nan")]
	savers_apr: Decimal,

	#[serde(rename = "saversDepth", with = "deserialize_decimal_with_nan")]
	savers_depth: Decimal,

	#[serde(rename = "saversUnits", with = "deserialize_decimal_with_nan")]
	savers_units: Decimal,

	status: PoolStatus,

	#[serde(rename = "synthSupply", with = "deserialize_decimal_with_nan")]
	synth_supply: Decimal,

	#[serde(rename = "synthUnits", with = "deserialize_decimal_with_nan")]
	synth_units: Decimal,

	#[serde(rename = "totalCollateral", with = "deserialize_decimal_with_nan")]
	total_collateral: Decimal,

	#[serde(rename = "totalDebtTor", with = "deserialize_decimal_with_nan")]
	total_debt_tor: Decimal,

	#[serde(with = "deserialize_decimal_with_nan")]
	units: Decimal,

	#[serde(rename = "volume24h", with = "deserialize_decimal_with_nan")]
	volume_24h: Decimal,
}

mod deserialize_decimal_with_nan {
	use rust_decimal::Decimal;

        #[allow(clippy::unnecessary_wraps, clippy::unnecessary_result_map_or_else)]
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		Ok(rust_decimal::serde::str::deserialize(deserializer).map_or_else(|_| Decimal::from(0), |d| d))
	}

	pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		rust_decimal::serde::str::serialize(value, serializer)
	}
}

impl Pool {
	#[must_use]
	pub const fn get_annual_percentage_rate(&self) -> &Decimal {
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
	pub const fn get_asset_price(&self) -> &Decimal {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &Decimal {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_earnings(&self) -> &Decimal {
		&self.earnings
	}

	#[must_use]
	pub const fn get_earnings_annual_as_percent_of_depth(&self) -> &Decimal {
		&self.earnings_annual_as_percent_of_depth
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &Decimal {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_lp_luvi(&self) -> &Decimal {
		&self.lp_luvi
	}

	#[must_use]
	pub const fn get_native_decimal(&self) -> &Decimal {
		&self.native_decimal
	}

	#[must_use]
	pub const fn get_pool_apy(&self) -> &Decimal {
		&self.pool_apy
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &Decimal {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_savers_apr(&self) -> &Decimal {
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

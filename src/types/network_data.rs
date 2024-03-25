use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::NetworkBlockRewards;
use crate::NetworkBondMetrics;

/*

*** Network Data Scheme ***

{
	"activeBonds": ["84861177346489", "86263771597472", ...],
	"activeNodeCount": "102",
	"blockRewards": NetworkBlockRewards,
	"bondMetrics": NetworkBondMetrics,
	"bondingAPY": "0.05095825272975807",
	"liquidityAPY": "0.3479067311754007",
	"nextChurnHeight": "15103050",
	"poolActivationCountdown": "17202",
	"poolShareFactor": "0.7503778666648501",
	"standbyBonds": ["100000001", "102118591", ...],
	"standbyNodeCount": "25",
	"totalPooledRune": "2477732809218387",
	"totalReserve": "8137240568156052"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkData {
	#[serde(rename = "activeBonds")]
	active_bonds: Vec<String>,

	#[serde(rename = "activeNodeCount", deserialize_with = "deserialize_number_from_string")]
	active_node_count: u64,

	#[serde(rename = "blockRewards")]
	block_rewards: NetworkBlockRewards,

	#[serde(rename = "bondMetrics")]
	bond_metrics: NetworkBondMetrics,

	#[serde(rename = "bondingAPY", with = "rust_decimal::serde::str")]
	bonding_apy: Decimal,

	#[serde(rename = "liquidityAPY", with = "rust_decimal::serde::str")]
	liquidity_apy: Decimal,

	#[serde(rename = "nextChurnHeight", deserialize_with = "deserialize_number_from_string")]
	next_churn_height: u64,

	#[serde(rename = "poolActivationCountdown", deserialize_with = "deserialize_number_from_string")]
	pool_activation_countdown: u64,

	#[serde(rename = "poolShareFactor", with = "rust_decimal::serde::str")]
	pool_share_factor: Decimal,

	#[serde(rename = "standbyBonds")]
	standby_bonds: Vec<String>,

	#[serde(rename = "standbyNodeCount", deserialize_with = "deserialize_number_from_string")]
	standby_node_count: u64,

	#[serde(rename = "totalPooledRune", deserialize_with = "deserialize_number_from_string")]
	total_pooled_rune: u64,

	#[serde(rename = "totalReserve", deserialize_with = "deserialize_number_from_string")]
	total_reserve: u64,
}

impl NetworkData {
	#[must_use]
	pub fn get_active_bonds(&self) -> Vec<u64> {
		let mut active_bonds: Vec<u64> = Vec::new();
		for bond in &self.active_bonds {
                        let b = bond.parse::<u64>().unwrap_or_default();
			active_bonds.push(b);
		}
		active_bonds
	}

	#[must_use]
	pub const fn get_active_node_count(&self) -> &u64 {
		&self.active_node_count
	}

	#[must_use]
	pub const fn get_block_rewards(&self) -> &NetworkBlockRewards {
		&self.block_rewards
	}

	#[must_use]
	pub const fn get_bond_metrics(&self) -> &NetworkBondMetrics {
		&self.bond_metrics
	}

	#[must_use]
	pub const fn get_bonding_apy(&self) -> &Decimal {
		&self.bonding_apy
	}

	#[must_use]
	pub const fn get_liquidity_apy(&self) -> &Decimal {
		&self.liquidity_apy
	}

	#[must_use]
	pub const fn get_next_churn_height(&self) -> &u64 {
		&self.next_churn_height
	}

	#[must_use]
	pub const fn get_pool_activation_countdown(&self) -> &u64 {
		&self.pool_activation_countdown
	}

	#[must_use]
	pub const fn get_pool_share_factor(&self) -> &Decimal {
		&self.pool_share_factor
	}

	#[must_use]
	pub fn get_standby_bonds(&self) -> Vec<u64> {
		let mut standby_bonds: Vec<u64> = Vec::new();
		for bond in &self.standby_bonds {
                        let b = bond.parse::<u64>().unwrap_or_default();
			standby_bonds.push(b);
		}
		standby_bonds
	}

	#[must_use]
	pub const fn get_standby_node_count(&self) -> &u64 {
		&self.standby_node_count
	}

	#[must_use]
	pub const fn get_total_pooled_rune(&self) -> &u64 {
		&self.total_pooled_rune
	}

	#[must_use]
	pub const fn get_total_reserve(&self) -> &u64 {
		&self.total_reserve
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn deserialize_network_data() {
		let data = json!({
			"activeBonds": [
				"84963273346489",
				"89263732456633",
				"89660184000000",
				"90081923360343",
				"91350491903044",
				"91405880588214",
				"91408541031233"
			],
			"activeNodeCount": "102",
			"blockRewards": {
				"blockReward": "193112485",
				"bondReward": "52684600",
				"poolReward": "140427884"
			},
			"bondMetrics": {
				"averageActiveBond": "97343657145099",
				"averageStandbyBond": "8705936525166",
				"bondHardCap": "99152166976715",
				"maximumActiveBond": "109057857825816",
				"maximumStandbyBond": "91654294023249",
				"medianActiveBond": "98415404058261",
				"medianStandbyBond": "318000000",
				"minimumActiveBond": "84963273346489",
				"minimumStandbyBond": "53920223428387",
				"totalActiveBond": "9929053028800114",
				"totalStandbyBond": "226354349654331"
			},
			"bondingAPY": "0.04739437154779713",
			"liquidityAPY": "0.25334146946310465",
			"nextChurnHeight": "15195210",
			"poolActivationCountdown": "11229",
			"poolShareFactor": "0.7271818014396857",
			"standbyBonds": [
				"100000001",
				"102118591",
				"109377729"
			],
			"standbyNodeCount": "26",
			"totalPooledRune": "2708826360727080",
			"totalReserve": "8119993778581423"
		})
		.to_string();

		let deserialized: NetworkData = serde_json::from_str(&data).unwrap();

		let active_bonds: Vec<u64> = vec![84963273346489, 89263732456633, 89660184000000, 90081923360343, 91350491903044, 91405880588214, 91408541031233];
		assert_eq!(deserialized.get_active_bonds(), active_bonds);
	}
}

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Network Block Rewards Scheme ***

{
		"blockReward": "193522654",
		"bondReward": "48307537",
		"poolReward": "145215116"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkBlockRewards {
	#[serde(rename = "blockReward", deserialize_with = "deserialize_number_from_string")]
	block_reward: u64,

	#[serde(rename = "bondReward", deserialize_with = "deserialize_number_from_string")]
	bond_reward: u64,

	#[serde(rename = "poolReward", deserialize_with = "deserialize_number_from_string")]
	pool_reward: u64,
}

impl NetworkBlockRewards {
	#[must_use]
	pub const fn get_block_reward(&self) -> &u64 {
		&self.block_reward
	}

	#[must_use]
	pub const fn get_bond_reward(&self) -> &u64 {
		&self.bond_reward
	}

	#[must_use]
	pub const fn get_pool_reward(&self) -> &u64 {
		&self.pool_reward
	}
}

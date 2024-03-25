use serde::{Deserialize, Serialize};

/*

*** Node Scheme ***

{
	"ed25519": "thorpub1addwnpepq2camh2ef7hncncu4pnzpkx95vdqtjl6jgaxadzm9u4gv65kew80slzt4vy",
	"nodeAddress": "thor1aw876sn7sdyllrzpnp0vekmcqwxnegl2mvy299",
	"secp256k1": "thorpub1addwnpepq2camh2ef7hncncu4pnzpkx95vdqtjl6jgaxadzm9u4gv65kew80slzt4vy"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
	ed25519: String,

	#[serde(rename = "nodeAddress")]
	node_address: String,

	secp256k1: String,
}

impl Node {
	#[must_use]
	pub const fn get_ed25519(&self) -> &String {
		&self.ed25519
	}

	#[must_use]
	pub const fn get_node_address(&self) -> &String {
		&self.node_address
	}

	#[must_use]
	pub const fn get_secp256k1(&self) -> &String {
		&self.secp256k1
	}
}

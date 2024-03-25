use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::Node;

/*

*** Nodes Scheme ***

[Node, Node, ...]

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeList(Vec<Node>);

impl NodeList {
	#[must_use]
	pub fn new(nodes: Vec<Node>) -> Self {
		Self(nodes)
	}

	#[must_use]
	pub const fn get_nodes(&self) -> &Vec<Node> {
		&self.0
	}

	#[must_use]
	pub fn get_ed25519s(&self) -> Vec<String> {
		let mut ed25519s = HashSet::new();
		for node in self.get_nodes() {
			ed25519s.insert(node.get_ed25519().to_string());
		}
		ed25519s.into_iter().collect()
	}

	#[must_use]
	pub fn get_node_addresses(&self) -> Vec<String> {
		let mut node_addresses = HashSet::new();
		for node in self.get_nodes() {
			node_addresses.insert(node.get_node_address().to_string());
		}
		node_addresses.into_iter().collect()
	}

	#[must_use]
	pub fn get_secp256k1s(&self) -> Vec<String> {
		let mut secp256k1s = HashSet::new();
		for node in self.get_nodes() {
			secp256k1s.insert(node.get_secp256k1().to_string());
		}
		secp256k1s.into_iter().collect()
	}
}

impl IntoIterator for NodeList {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = Node;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

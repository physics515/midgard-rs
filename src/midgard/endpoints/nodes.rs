use anyhow::Result;
use chrono::Utc;

use crate::Midgard;
use crate::{api_get_node_list, NodeList};

impl Midgard {
	/// Returns a list of Node public keys and adresses.
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	/// let node_list = midgard.get_node_list().await.unwrap();
	///
	/// assert!(!node_list.get_nodes().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_node_list(&mut self) -> Result<NodeList> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_node_list(self.get_config().get_base_url()).await
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_node_list() {
		let mut midgard = Midgard::new();
		let node_list = midgard.get_node_list().await.unwrap();

		println!("node list: {}", json!(node_list).to_string());
		assert!(!node_list.get_nodes().is_empty());
	}
}

use anyhow::Result;
use chrono::Utc;

use crate::{api_get_balance, Balance, Midgard};

impl Midgard {
	/// Returns all coin amounts of the given address at the specified timestamp or height, or at the latest process block if neither is provided.
	/// Only one of timestamp or height can be specified, not both, if both are specified the request will fail.
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::Midgard;
	///# tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	/// let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
	/// let balance = midgard.get_balance(address, None, None).await.unwrap();
	/// assert!(*balance.get_height() > 0);
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_balance(&mut self, address: &str, timestamp: Option<i64>, height: Option<u64>) -> Result<Balance> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_balance(self.get_config().get_base_url(), address, timestamp, height).await
	}
}

#[cfg(test)]
mod tests {
	use rand::prelude::*;
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_balance() {
		let mut midgard = Midgard::new();

		let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
		let balance = midgard.get_balance(address, None, None).await.unwrap();

		println!("{}", json!(&balance));
		assert!(*balance.get_height() > 0);
	}
}

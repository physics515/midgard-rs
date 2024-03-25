use anyhow::Result;
use chrono::Utc;

use crate::{api_get_global_stats, GlobalStats, Midgard};

impl Midgard {
	/// Returns an object containing global stats for all pools and all transactions.
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::Midgard;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get the global stats
	/// let global_stats = midgard.get_global_stats().await.unwrap();
	///
	/// assert!(*global_stats.get_add_liquidity_count() > 0);
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_global_stats(&mut self) -> Result<GlobalStats> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_global_stats(self.get_config().get_base_url()).await
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_global_stats() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get the global stats
		let global_stats = midgard.get_global_stats().await.unwrap();

		println!("global_stats: {}", json!(global_stats));

		assert!(*global_stats.get_add_liquidity_count() > 0);
	}
}

use anyhow::Result;
use chrono::Utc;

use crate::Midgard;
use crate::{api_get_health_info, HealthInfo};

impl Midgard {
	/// Returns an object containing the health response of the API. Meaning of heights:
	/// * lastThorNode - Latest block as reported by `ThorNode`.
	/// * lastFetched - Latest block fetched from `ThorNode`.
	/// * lastCommitted - Latest block committed to the DB but not fully processed yet.
	/// * lastAggregated - Latest block fully processed and aggregated.
	/// * genesisInfo - The genesis height Midgard bootstrapped with.
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use serde_json::json;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get the health info
	/// let health_info = midgard.get_health_info().await.unwrap();
	///
	/// println!("health_info: {}", json!(health_info));
	///
	/// assert!(health_info.get_last_thor_node().is_some());
	/// assert!(health_info.get_last_fetched().is_some());
	/// assert!(health_info.get_last_committed().is_some());
	/// assert!(health_info.get_last_aggregated().is_some());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_health_info(&mut self) -> Result<HealthInfo> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_health_info(self.get_config().get_base_url()).await
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use crate::Midgard;

	#[tokio::test]
	async fn test_get_health_info() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get the health info
		let health_info = midgard.get_health_info().await.unwrap();

		println!("health_info: {}", json!(health_info));

		assert!(health_info.get_last_thor_node().is_some());
		assert!(health_info.get_last_fetched().is_some());
		assert!(health_info.get_last_committed().is_some());
		assert!(health_info.get_last_aggregated().is_some());
	}
}

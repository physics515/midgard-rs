use chrono::Utc;

use crate::APIError;
use crate::Midgard;
use crate::{api_get_network_data, NetworkData};

impl Midgard {
	/// Returns an object containing Network data
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	/// let network_data = midgard.get_network_data().await.unwrap();
	///
	/// assert!(!network_data.get_active_bonds().is_empty());
	/// # });
	/// ```
	///
	/// # Errors
	///
	/// Returns [`crate::APIError::ReqwestError`] if the request to the Midgard
	/// instance could not be made or its body could not be read.
	///
	/// Returns [`crate::APIError::SerdeError`] if the response body is not the JSON
	/// this crate expects — which, because the HTTP status is not yet
	/// checked, is also what an error page from the instance looks like.
	pub async fn get_network_data(&mut self) -> Result<NetworkData, APIError> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_network_data(self.get_config().get_base_url()).await
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_network_data() {
		let mut midgard = Midgard::new();
		let network_data = midgard.get_network_data().await.unwrap();

		println!("network data: {}", json!(network_data).to_string());
		assert!(!network_data.get_active_bonds().is_empty());
	}
}

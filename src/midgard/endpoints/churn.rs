use anyhow::Result;
use chrono::Utc;

use crate::{api_get_churn_list, ChurnsList, Midgard};

impl Midgard {
	/// Returns block height and timestamp for each churn.
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
	/// // Get the churn list
	/// let churn_list = midgard.get_churn_list().await.unwrap();
	///
	/// assert!(!churn_list.get_churns().is_none());
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
	pub async fn get_churn_list(&mut self) -> Result<ChurnsList> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_churn_list(self.get_config().get_base_url()).await
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_churn_list() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get the churn list
		let churn_list = midgard.get_churn_list().await.unwrap();

		println!("churn_list: {}", json!(churn_list));

		assert!(!churn_list.get_churns().is_none());
	}
}

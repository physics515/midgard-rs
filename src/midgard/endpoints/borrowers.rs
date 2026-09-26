use chrono::Utc;

use crate::APIError;
use crate::{api_get_borrowers_details, api_get_borrowers_list, BorrowersDetails, BorrowersList, Midgard};

impl Midgard {
	/// Returns an array of statistics for all the open loans associated with a given borrower address.
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
	/// // Details are requested for a comma-separated list of addresses
	/// let borrowers = midgard.get_borrowers_list(None).await.unwrap();
	/// let address = borrowers.get_borrowers().iter().take(2).cloned().collect::<Vec<_>>().join(",");
	///
	/// // Get the borrowers details
	/// let borrowers_details = midgard.get_borrowers_details(&address).await.unwrap();
	///
	/// assert!(!borrowers_details.get_pools().is_empty());
	/// # });
	/// ```
	///
	/// # Errors
	///
	/// Returns [`crate::APIError::ReqwestError`] if the request to the Midgard
	/// instance could not be made or its body could not be read.
	///
	/// Returns [`crate::APIError::HttpStatus`] if the instance answered with a
	/// non-success status, carrying the code, the URL and a truncated body.
	///
	/// Returns [`crate::APIError::SerdeError`] if the response body is not the JSON
	/// this crate expects.
	pub async fn get_borrowers_details(&mut self, address: &str) -> Result<BorrowersDetails, APIError> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_borrowers_details(self.get_config().get_base_url(), address).await
	}

	/// Returns an array containing the addresses for all borrowers. Addresses are only shown once.
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
	/// // Set the asset
	/// let asset = Some("BTC.BTC".to_string());
	///
	/// // Get the borrowers list
	/// let borrowers_list = midgard.get_borrowers_list(asset).await.unwrap();
	///
	/// assert!(!borrowers_list.get_borrowers().is_empty());
	/// # });
	/// ```
	///
	/// # Errors
	///
	/// Returns [`crate::APIError::ReqwestError`] if the request to the Midgard
	/// instance could not be made or its body could not be read.
	///
	/// Returns [`crate::APIError::HttpStatus`] if the instance answered with a
	/// non-success status, carrying the code, the URL and a truncated body.
	///
	/// Returns [`crate::APIError::SerdeError`] if the response body is not the JSON
	/// this crate expects.
	///
	/// Returns a `serde_urlencoded` error if the query parameters could not
	/// be encoded.
	pub async fn get_borrowers_list(&mut self, asset: Option<String>) -> Result<BorrowersList, APIError> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_borrowers_list(self.get_config().get_base_url(), asset).await
	}
}

#[cfg(test)]
mod tests {
	use rand::prelude::*;
	use serde_json::json;

	use super::*;
	use crate::test_support::seeded_rng;

	#[tokio::test]
	async fn test_get_borrowers_details() {
		let mut rng = seeded_rng();
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Two borrowers, chosen by the seeded generator
		let borrowers = midgard.get_borrowers_list(None).await.unwrap();
		let address_1 = borrowers.get_borrowers().choose(&mut rng).unwrap().clone();
		let address_2 = borrowers.get_borrowers().choose(&mut rng).unwrap().clone();
		let address = vec![address_1, address_2].join(",");

		// Get the borrowers details
		let borrowers_details = midgard.get_borrowers_details(&address).await.unwrap();

		println!("borrowers_details: {}", json!(borrowers_details));

		assert!(!borrowers_details.get_pools().is_empty());
	}

	#[tokio::test]
	async fn test_get_borrowers_list() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Set the asset
		let asset = Some("BTC.BTC".to_string());

		// Get the borrowers list
		let borrowers_list = midgard.get_borrowers_list(asset).await.unwrap();

		println!("borrowers_list: {}", json!(borrowers_list));

		assert!(!borrowers_list.get_borrowers().is_empty());
	}
}

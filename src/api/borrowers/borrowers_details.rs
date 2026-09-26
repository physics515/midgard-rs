
use crate::{APIError, BorrowersDetails};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_borrowers_details(base_url: &str, address: &str) -> Result<BorrowersDetails, APIError> {
	let endpoint = base_url.to_string() + "borrower/" + address;

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: BorrowersDetails = serde_json::from_str(&response)?;

	Ok(res)
}

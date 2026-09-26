use crate::{APIError, SaversDetails};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_savers_details(base_url: &str, address: &[String]) -> Result<SaversDetails, APIError> {
	let address = address.join(",");
	let endpoint = base_url.to_string() + "saver/" + &address;

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: SaversDetails = serde_json::from_str(&response)?;

	Ok(res)
}

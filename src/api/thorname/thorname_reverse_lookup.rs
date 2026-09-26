
use crate::{APIError, ThornameReverseLookup};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_thorname_reverse_lookup(base_url: &str, address: &str) -> Result<ThornameReverseLookup, APIError> {
	let endpoint = base_url.to_string() + "thorname/rlookup/" + address;

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: ThornameReverseLookup = serde_json::from_str(&response)?;

	Ok(res)
}

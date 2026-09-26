
use crate::{APIError, ThornameDetails};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_thorname_details(base_url: &str, name: &str) -> Result<ThornameDetails, APIError> {
	let endpoint = base_url.to_string() + "thorname/lookup/" + name;

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: ThornameDetails = serde_json::from_str(&response)?;

	Ok(res)
}

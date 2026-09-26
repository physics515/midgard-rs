
use crate::{APIError, KnownPoolList};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_known_pool_list(base_url: &str) -> Result<KnownPoolList, APIError> {
	let endpoint = base_url.to_string() + "knownpools";
	let response = crate::api::http::get_body(&endpoint).await?;

	let res: KnownPoolList = serde_json::from_str(&response)?;

	Ok(res)
}

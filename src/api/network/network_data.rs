use crate::{APIError, NetworkData};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_network_data(base_url: &str) -> Result<NetworkData, APIError> {
	let endpoint = base_url.to_string() + "network";
	let response = crate::api::http::get_body(&endpoint).await?;

	let res: NetworkData = serde_json::from_str(&response)?;

	Ok(res)
}

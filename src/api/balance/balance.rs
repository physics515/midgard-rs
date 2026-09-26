use crate::{APIError, Balance};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_balance(base_url: &str, address: &str, timestamp: Option<i64>, height: Option<u64>) -> Result<Balance, APIError> {
	let mut endpoint = base_url.to_string() + "balance/" + address;
	if timestamp.is_some() && height.is_some() {
		return Err(APIError::InvalidParameter("Only one of timestamp or height can be specified, not both, if both are specified the request will fail.".to_string()));
	}
	if timestamp.is_some() || height.is_some() {
		endpoint.push('?');
		if let Some(timestamp) = timestamp {
			endpoint.push_str(&serde_urlencoded::to_string([("timestamp", timestamp.to_string())])?);
			endpoint.push('&');
		}
		if let Some(height) = height {
			endpoint.push_str(&serde_urlencoded::to_string([("height", height.to_string())])?);
		}
	}

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: Balance = serde_json::from_str(&response)?;

	Ok(res)
}

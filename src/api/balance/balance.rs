use anyhow::{bail, Result};

use crate::{APIError, Balance};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_balance(base_url: &str, address: &str, timestamp: Option<i64>, height: Option<u64>) -> Result<Balance> {
	let mut endpoint = base_url.to_string() + "balance/" + address;
	if timestamp.is_some() && height.is_some() {
		bail!(APIError::InvalidParameter("Only one of timestamp or height can be specified, not both, if both are specified the request will fail.".to_string()));
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

	let response = match reqwest::get(&endpoint).await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let res: Balance = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

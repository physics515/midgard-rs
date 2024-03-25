use anyhow::{bail, Result};

use crate::{APIError, BorrowersList};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_borrowers_list(base_url: &str, asset: Option<String>) -> Result<BorrowersList> {
	let mut endpoint = base_url.to_string() + "borrowers";
	if let Some(asset) = asset {
		endpoint.push('?');
		endpoint.push_str(&serde_urlencoded::to_string([("asset", asset)])?);
	}

	let response = match reqwest::get(&endpoint).await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let res: BorrowersList = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

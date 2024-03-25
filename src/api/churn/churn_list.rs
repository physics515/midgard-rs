use anyhow::{bail, Result};

use crate::{APIError, ChurnsList};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_churn_list(base_url: &str) -> Result<ChurnsList> {
	let endpoint = base_url.to_string() + "churns";
	let response = match reqwest::get(&endpoint).await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let res: ChurnsList = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

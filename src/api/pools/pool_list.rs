use anyhow::{bail, Result};

use crate::{APIError, PoolList, PoolStatus, TimePeriod};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_pool_list(base_url: &str, status: Option<PoolStatus>, period: Option<TimePeriod>) -> Result<PoolList> {
	let period = period.unwrap_or_default();

	let mut params = vec![];
	if let Some(status) = status {
		params.push(("status", status.to_string()));
	}

	if period != TimePeriod::FourteenDays {
		params.push(("period", period.to_string()));
	}

	let mut endpoint = base_url.to_string() + "pools";
	if !params.is_empty() {
		endpoint.push('?');
		endpoint.push_str(&serde_urlencoded::to_string(params)?);
	}

	let response = match reqwest::get(&endpoint).await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let res: PoolList = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

use anyhow::{bail, Result};

use crate::{APIError, PoolStatistics, TimePeriod};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_statistics_of_pool(base_url: &str, pool: &str, period: Option<TimePeriod>) -> Result<PoolStatistics> {
	let period = period.unwrap_or_default();

	let mut endpoint = base_url.to_string() + "pool/" + pool + "/stats";
	if period != TimePeriod::FourteenDays {
		endpoint.push('?');
		endpoint.push_str(&serde_urlencoded::to_string([("period", period.to_string())])?);
	}

	let response = match reqwest::get(&endpoint).await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let res: PoolStatistics = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

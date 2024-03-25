use anyhow::{bail, Result};

use crate::{APIError, Interval, TVLHistory};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_total_value_locked_history(base_url: &str, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<TVLHistory> {
	let mut endpoint = base_url.to_string() + "history/tvl";
	if interval.is_some() || count.is_some() {
		endpoint.push('?');
		if let Some(interval) = interval {
			endpoint.push_str(&serde_urlencoded::to_string([("interval", interval.to_string())])?);
			endpoint.push('&');
		}
		if let Some(count) = count {
			if !(1..=400).contains(&count) {
				bail!(APIError::InvalidParameter("count".to_string()));
			}
			endpoint.push_str(&serde_urlencoded::to_string([("count", count.to_string())])?);
			endpoint.push('&');
		}
		if let Some(to) = to {
			endpoint.push_str(&serde_urlencoded::to_string([("to", to.to_string())])?);
			endpoint.push('&');
		}
		if let Some(from) = from {
			endpoint.push_str(&serde_urlencoded::to_string([("from", from.to_string())])?);
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

	let res: TVLHistory = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}


use crate::{APIError, Interval, SwapHistory};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_swaps_history(base_url: &str, pool: Option<&str>, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<SwapHistory, APIError> {
	let mut endpoint = base_url.to_string() + "history/swaps";
	if interval.is_some() || count.is_some() {
		endpoint.push('?');
		if let Some(interval) = interval {
			endpoint.push_str(&serde_urlencoded::to_string([("interval", interval.to_string())])?);
			endpoint.push('&');
		}
		if let Some(count) = count {
			if !(1..=400).contains(&count) {
				return Err(APIError::InvalidParameter("count".to_string()));
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
			endpoint.push('&');
		}
		if let Some(pool) = pool {
			endpoint.push_str(&serde_urlencoded::to_string([("pool", pool.to_string())])?);
		}
	}

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: SwapHistory = serde_json::from_str(&response)?;

	Ok(res)
}

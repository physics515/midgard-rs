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

/* #[cfg(test)]
mod tests {
	use rand::prelude::*;
	use serde_json::json;

	use super::*;
	use crate::api_get_pool_list;

	#[tokio::test]
	async fn test_get_statistics_of_pool() {
		let pool_list = api_get_pool_list(None, None).await.unwrap();
		let random_usize = thread_rng().gen_range(0..pool_list.get_assets().len());
		let pool = pool_list.get_assets()[random_usize].clone();

		let pool_statistics = api_get_statistics_of_pool(&pool, None).await.unwrap();
		println!("{}", json!(pool_statistics));
		assert!(!pool_statistics.get_asset().is_empty());
	}
} */

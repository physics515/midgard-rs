use crate::{APIError, Pool, TimePeriod};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_details_of_pool(base_url: &str, pool: &str, period: Option<TimePeriod>) -> Result<Pool, APIError> {
	let period = period.unwrap_or_default();

	let mut endpoint = base_url.to_string() + "pool/" + pool;
	if period != TimePeriod::FourteenDays {
		endpoint.push('?');
		endpoint.push_str(&serde_urlencoded::to_string([("period", period.to_string())])?);
	}

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: Pool = serde_json::from_str(&response)?;

	Ok(res)
}

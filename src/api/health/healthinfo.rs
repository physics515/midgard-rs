use anyhow::{bail, Result};

use crate::{APIError, HealthInfo};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_health_info(base_url: &str) -> Result<HealthInfo> {
	let endpoint = base_url.to_string() + "health";
	let response = crate::api::http::get_body(&endpoint).await?;

	let res: HealthInfo = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

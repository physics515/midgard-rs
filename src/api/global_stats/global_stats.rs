use anyhow::{bail, Result};

use crate::{APIError, GlobalStats};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_global_stats(base_url: &str) -> Result<GlobalStats> {
	let endpoint = base_url.to_string() + "stats";
	let response = crate::api::http::get_body(&endpoint).await?;

	let res: GlobalStats = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

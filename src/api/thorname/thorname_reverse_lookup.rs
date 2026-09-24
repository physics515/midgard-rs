use anyhow::{bail, Result};

use crate::{APIError, ThornameReverseLookup};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_thorname_reverse_lookup(base_url: &str, address: &str) -> Result<ThornameReverseLookup> {
	let endpoint = base_url.to_string() + "thorname/rlookup/" + address;

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: ThornameReverseLookup = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}


use crate::{APIError, ChurnsList};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_churn_list(base_url: &str) -> Result<ChurnsList, APIError> {
	let endpoint = base_url.to_string() + "churns";
	let response = crate::api::http::get_body(&endpoint).await?;

	let res: ChurnsList = serde_json::from_str(&response)?;

	Ok(res)
}

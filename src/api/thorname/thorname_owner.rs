use crate::{APIError, ThornameOwner};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_thorname_owner(base_url: &str, address: &str) -> Result<ThornameOwner, APIError> {
	let endpoint = base_url.to_string() + "thorname/owner/" + address;

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: ThornameOwner = serde_json::from_str(&response)?;

	Ok(res)
}

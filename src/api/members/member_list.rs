use anyhow::{bail, Result};

use crate::{APIError, MemberList};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_member_list(base_url: &str, pool: Option<String>) -> Result<MemberList> {
	let mut endpoint = base_url.to_string() + "members";
	if let Some(pool) = pool {
		endpoint.push('?');
		endpoint.push_str(&serde_urlencoded::to_string([("pool", pool)])?);
	}

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: MemberList = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

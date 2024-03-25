use anyhow::{bail, Result};

use crate::{APIError, MemberDetails};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_member_details(base_url: &str, address: &str, show_savers: bool) -> Result<MemberDetails> {
	let mut endpoint = base_url.to_string() + "member/" + address;
	endpoint.push('?');
	endpoint.push_str(&serde_urlencoded::to_string([("showSavers", show_savers)])?);

	let response = match reqwest::get(&endpoint).await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	println!("response: {}", response);

	let res: MemberDetails = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

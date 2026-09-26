
use crate::{APIError, MemberDetails};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_member_details(base_url: &str, address: &str, show_savers: bool) -> Result<MemberDetails, APIError> {
	let mut endpoint = base_url.to_string() + "member/" + address;
	endpoint.push('?');
	endpoint.push_str(&serde_urlencoded::to_string([("showSavers", show_savers)])?);

	let response = crate::api::http::get_body(&endpoint).await?;

	let res: MemberDetails = serde_json::from_str(&response)?;

	Ok(res)
}

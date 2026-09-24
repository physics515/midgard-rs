use anyhow::{bail, Result};

use crate::{APIError, NodeList};

/// # Errors
/// 1. Network Request Failed
/// 2. JSON Parsing Error
/// 3. Faild to Parse URL Parameters
#[allow(clippy::module_name_repetitions)]
pub async fn api_get_node_list(base_url: &str) -> Result<NodeList> {
	let endpoint = base_url.to_string() + "nodes";
	let response = crate::api::http::get_body(&endpoint).await?;

	let res: NodeList = match serde_json::from_str(&response) {
		Ok(res) => res,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(res)
}

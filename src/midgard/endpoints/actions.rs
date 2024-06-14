use anyhow::Result;
use chrono::Utc;

use crate::Midgard;
use crate::{api_get_action_list, ActionList, GetActionList};

impl Midgard {
	/// List actions along with their related transactions. An action is generated by one or more inbound transactions with the intended action set in the transaction memo. The action may result in one or more outbound transactions. Results are paginated by sets of 50. Filters may be applied to query actions.
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::GetActionList;
	/// use midgard_rs::Midgard;
	///
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	///
	/// let params = GetActionList::new(vec!["BTC.BTC".to_string()], 10);
	/// let actions = midgard.get_actions(params).await.unwrap();
	///
	/// assert!(!actions.get_actions().get_actions().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `next_page_token` from the previous response to the next request.
	/// ```rust
	/// use midgard_rs::GetActionList;
	/// use midgard_rs::Midgard;
	///
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	///
	/// let mut params = GetActionList::new(vec!["BTC.BTC".to_string()], 10);
	/// let actions = midgard.get_actions(params.clone()).await.unwrap();
	///
	/// assert!(!actions.get_actions().get_actions().is_empty());
	///
	///
	/// let next_page_token = actions.get_meta().get_next_page_token();
	/// if let Some(next_page_token) = next_page_token {
	///         params.set_next_page_token(next_page_token);
	///         let next_actions = midgard.get_actions(params).await.unwrap();
	///         assert!(!next_actions.get_actions().get_actions().is_empty());
	/// }
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_actions(&mut self, params: GetActionList) -> Result<ActionList> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_action_list(self.get_config().get_base_url(), params).await
	}
}

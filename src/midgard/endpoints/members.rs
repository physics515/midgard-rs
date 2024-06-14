use anyhow::Result;
use chrono::Utc;

use crate::{api_get_member_details, api_get_member_list, MemberDetails, MemberList, Midgard};

impl Midgard {
	/// Returns an array of statistics for all the liquidity providers associated with a given member address.
	/// ## path Parameters
	/// `address`
	/// * required
	/// * string
	///   * Example: bnb1jxfh2g85q3v0tdq56fnevx6xcxtcnhtsmcu64m,bc1qcxssye4j6730h7ehgega3gyykkuwgdgmmpu62n
	/// * Address to match liquidity providers. Either a rune or an asset address may be given. Query can also be multiple addresses should be seperated by comma (',')
	///
	/// ## query Parameters
	/// `showSavers`
	/// * boolean
	/// * Default: false
	///   * Example: showSavers=true
	/// * A flag to show saver vault membership details, the default is false.
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::MemberDetails;
	/// use rand::prelude::*;
	///
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	///
	/// // get random pool members list
	/// let pool_list = midgard.get_pool_list(None, None).await.unwrap();
	/// let random_usize = thread_rng().gen_range(0..pool_list.get_assets().len());
	/// let pool = pool_list.get_assets()[random_usize].clone();
	/// let member_list = midgard.get_member_list(Some(pool)).await.unwrap();
	///
	/// // get random member addresses
	/// let random_usize = thread_rng().gen_range(0..member_list.get_members().len());
	/// let random_usize_2 = thread_rng().gen_range(0..member_list.get_members().len());
	/// let address = vec![member_list.get_members()[random_usize].clone(), member_list.get_members()[random_usize_2].clone()];
	///
	/// // get member details
	/// let show_savers = false;
	/// let member_details = midgard.get_member_details(&address, show_savers).await.unwrap();
	///
	/// assert!(!member_details.get_pools().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_member_details(&mut self, address: &[String], show_savers: bool) -> Result<MemberDetails> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		let address = address.join(",");

		self.set_last_call(Utc::now());
		api_get_member_details(self.get_config().get_base_url(), &address, show_savers).await
	}

	/// Returns an array containing the addresses for all pool members. Addresses are only shown once. If there's both a RUNE address and an asset address for a member, only the RUNE address will be shown.
	/// ## query Parameters
	/// `pool`
	/// * string
	///  * Example: pool=BNB.BNB
	///
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::MemberList;
	/// use rand::prelude::*;
	///
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	///
	/// // get random pool
	/// let pool_list = midgard.get_pool_list(None, None).await.unwrap();
	/// let random_usize = thread_rng().gen_range(0..pool_list.get_assets().len());
	/// let pool = pool_list.get_assets()[random_usize].clone();
	///
	/// // get member list
	/// let member_list = midgard.get_member_list(Some(pool)).await.unwrap();
	///
	/// assert!(!member_list.get_members().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_member_list(&mut self, pool: Option<String>) -> Result<MemberList> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_member_list(self.get_config().get_base_url(), pool).await
	}
}

#[cfg(test)]
mod tests {
	use rand::prelude::*;
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_member_details() {
		let mut midgard = Midgard::new();

		// get random pool members list
		let pool_list = midgard.get_pool_list(None, None).await.unwrap();
		let random_usize = thread_rng().gen_range(0..pool_list.get_assets().len());
		let pool = pool_list.get_assets()[random_usize].clone();
		let member_list = midgard.get_member_list(Some(pool)).await.unwrap();

		// get random member addresses
		let random_usize = thread_rng().gen_range(0..member_list.get_members().len());
		let random_usize_2 = thread_rng().gen_range(0..member_list.get_members().len());
		let address = vec![member_list.get_members()[random_usize].clone(), member_list.get_members()[random_usize_2].clone()];

		// get member details
		let show_savers = false;
		let member_details = midgard.get_member_details(&address, show_savers).await.unwrap();

		assert!(!member_details.get_pools().is_empty());
	}

	#[tokio::test]
	async fn test_get_member_list() {
		let mut midgard = Midgard::new();

		// get random pool
		let pool_list = midgard.get_pool_list(None, None).await.unwrap();
		let random_usize = thread_rng().gen_range(0..pool_list.get_assets().len());
		let pool = pool_list.get_assets()[random_usize].clone();

		// get member list
		let member_list = midgard.get_member_list(Some(pool)).await.unwrap();

		println!("{}", json!(member_list.get_members()));
		assert!(!member_list.get_members().is_empty());
	}
}

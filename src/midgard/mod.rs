use chrono::Duration;
use chrono::{DateTime, Utc};
pub use config::*;
use serde::{Deserialize, Serialize};

mod config;
mod endpoints;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Midgard {
	config: Configuration,
	last_call: DateTime<Utc>,
}

impl Midgard {
        #[must_use]
	pub fn new() -> Self {
		Self { config: Configuration::default(), last_call: Utc::now() }
	}

        #[must_use]
	pub fn with_config(config: Configuration) -> Self {
		Self { config, last_call: Utc::now() }
	}

        #[must_use]
	pub const fn get_config(&self) -> &Configuration {
		&self.config
	}

	pub fn set_config(&mut self, config: Configuration) {
		self.config = config;
	}

        #[must_use]
	pub const fn get_last_call(&self) -> DateTime<Utc> {
		self.last_call
	}

	fn set_last_call(&mut self, last_call: DateTime<Utc>) {
		self.last_call = last_call;
	}

	/// Returns a future timestamp of when it is ok to call the Midgard API again
	fn ok_to_call_at(&self) -> DateTime<Utc> {
		let rate_limit: i64 = i64::try_from(self.config.get_rate_limit_ms()).map_or(1000, |rate_limit| rate_limit);
		let rate_limit = Duration::try_milliseconds(rate_limit).map_or_else(Duration::zero, |rate_limit| rate_limit);
		self.last_call.checked_add_signed(rate_limit).map_or_else(Utc::now, |res| res)
	}

	/// Sleeps until it is ok to call the Midgard API again
	async fn sleep_until_ok_to_call(&mut self) {
		let now = Utc::now();
		let ok_to_call_at = self.ok_to_call_at();
		if now < ok_to_call_at {
			let sleep_duration = ok_to_call_at - now;
			tokio::time::sleep(sleep_duration.to_std().unwrap()).await;
		}
		self.set_last_call(Utc::now());
	}
}


#[cfg(test)]
mod tests {
        use rand::prelude::*;
	use serde_json::json;
        use crate::GetActionList;

	use super::*;

        #[tokio::test]
        async fn endpoints() {
                let mut midgard = Midgard::new();


                // actions
                let params = GetActionList::new(vec!["BTC.BTC".to_string()], 10);
		let actions = midgard.get_actions(params).await.unwrap();
                assert!(!actions.get_actions().get_actions().is_empty());

                
                // action pagination
                let pool_list = midgard.get_pool_list(None, None).await.unwrap();
		let random_usize = thread_rng().gen_range(0..pool_list.get_assets().len());
		let pool = pool_list.get_assets()[random_usize].clone();

		let mut params = GetActionList::new(vec![pool.clone()], 5);
		let actions = midgard.get_actions(params.clone()).await.unwrap();
		assert!(!actions.get_actions().get_actions().is_empty());

		let next_page_token = actions.get_meta().get_next_page_token();
		if let Some(next_page_token) = next_page_token {
			params.set_next_page_token(next_page_token);
			let next_actions = midgard.get_actions(params).await.unwrap();

			assert!(!next_actions.get_actions().get_actions().is_empty());
		}

                // savers pools
                let pool_list = midgard.get_pool_list(None, None).await.unwrap();
                let savers_pools = pool_list.get_savers_pools();
                assert!(!savers_pools.is_empty());

                println!("savers pools: {}", json!(savers_pools));
        }

}
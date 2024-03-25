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

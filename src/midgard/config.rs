use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
	base_url: String,
	rate_limit_ms: u64,
}

impl Configuration {
        #[must_use]
	pub const fn new(base_url: String, rate_limit_ms: u64) -> Self {
		Self { base_url, rate_limit_ms }
	}

        #[must_use]
	pub fn get_base_url(&self) -> &str {
		&self.base_url
	}

        #[must_use]
	pub const fn get_rate_limit_ms(&self) -> u64 {
		self.rate_limit_ms
	}

	pub fn set_base_url(&mut self, base_url: String) {
		self.base_url = base_url;
	}

	pub fn set_rate_limit_ms(&mut self, rate_limit_ms: u64) {
		self.rate_limit_ms = rate_limit_ms;
	}
}

impl Default for Configuration {
	fn default() -> Self {
		Self { base_url: "https://midgard.ninerealms.com/v2/".to_string(), rate_limit_ms: 1000 }
	}
}

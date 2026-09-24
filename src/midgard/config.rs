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

/// The default Midgard v2 base URL: the public gateway recommended by the
/// `THORChain` developer documentation at
/// <https://dev.thorchain.org/concepts/connecting-to-thorchain.html>.
///
/// The two hosts this crate previously pointed at are both unusable as of
/// 2026-09-24: `midgard.ninerealms.com` no longer resolves at all (no A,
/// AAAA or CNAME record), and `midgard.thorswap.net` answers non-browser
/// clients with a Cloudflare interstitial challenge rather than JSON.
///
/// Liquify rate-limits this gateway to 50,000 requests per day per IP.
pub const DEFAULT_BASE_URL: &str = "https://gateway.liquify.com/chain/thorchain_midgard/v2/";

impl Default for Configuration {
	fn default() -> Self {
		Self { base_url: DEFAULT_BASE_URL.to_string(), rate_limit_ms: 1000 }
	}
}

#[cfg(test)]
mod tests {
	use super::{Configuration, DEFAULT_BASE_URL};

	/// The default base URL must be an absolute `https` URL ending in a
	/// slash: every endpoint builds its path by string concatenation
	/// (`base_url.to_string() + "health"`), so a missing trailing slash
	/// silently produces `.../v2health`.
	#[test]
	fn default_base_url_is_well_formed() {
		assert!(DEFAULT_BASE_URL.starts_with("https://"), "base url must be https: {DEFAULT_BASE_URL}");
		assert!(DEFAULT_BASE_URL.ends_with('/'), "base url must end in a slash: {DEFAULT_BASE_URL}");
		assert!(DEFAULT_BASE_URL.contains("/v2/"), "base url must address the v2 API: {DEFAULT_BASE_URL}");
	}

	#[test]
	fn default_configuration_uses_default_base_url() {
		let config = Configuration::default();
		assert_eq!(config.get_base_url(), DEFAULT_BASE_URL);
		assert_eq!(config.get_rate_limit_ms(), 1000);
	}

	/// `Configuration::new` is public API and must not be overridden by the
	/// default, so a consumer can always point the client at their own node.
	#[test]
	fn new_overrides_the_default() {
		let mut config = Configuration::new("https://example.invalid/v2/".to_string(), 250);
		assert_eq!(config.get_base_url(), "https://example.invalid/v2/");
		assert_eq!(config.get_rate_limit_ms(), 250);

		config.set_base_url("https://other.invalid/v2/".to_string());
		config.set_rate_limit_ms(500);
		assert_eq!(config.get_base_url(), "https://other.invalid/v2/");
		assert_eq!(config.get_rate_limit_ms(), 500);
	}
}

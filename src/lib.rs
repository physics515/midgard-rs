#![warn(clippy::pedantic, clippy::nursery, clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

//! # Midgard
//! Consumer information relating to swaps, pools, and volume. Midgard returns time-series information regarding the `THORChain` network, such as volume, pool information, users, liquidity providers and more. It also proxies to `THORNode` to reduce burden on the network. Runs on every node.
//!
//! ## midgard-rs
//! A typed Rust client for the `THORChain` Midgard v2 API. Every response is
//! deserialized into a concrete struct rather than handed back as loose JSON,
//! and every failure is an [`APIError`] you can match on.
//!
//! ## Coverage
//!
//! **25 of the 39 data endpoints**, checked against Midgard OpenAPI spec 2.35.0
//! on 2026-09-25. Not yet implemented: `/v2/bonds/{address}`, `/v2/holders`,
//! `/v2/votes`, `/v2/runepool/{address}`, `/v2/tcy/distribution/{address}`, the
//! three `/v2/history/affiliate*` endpoints, `/v2/history/reserve`,
//! `/v2/history/rune`, `/v2/history/runepool`, and the three `/v2/metrics/*`
//! endpoints.
//!
//! ## Where your requests go
//!
//! By default the client calls
//! `https://gateway.liquify.com/chain/thorchain_midgard/v2/` — a **third-party
//! public gateway**, the one recommended by the [`THORChain` developer
//! documentation](https://dev.thorchain.org/concepts/connecting-to-thorchain.html),
//! rate limited to 50,000 requests per day per IP. Your queries reach Liquify's
//! infrastructure, not a node you control. That default is a convenience, not an
//! endorsement: point the client at your own Midgard instance, or any other
//! public one, with a [`Configuration`]. The default is exported as
//! [`DEFAULT_BASE_URL`].
//!
//! The client is rate limited to 1 request per second by default, which
//! [`Configuration`] also controls.
//!
//! ## Basic Usage
//!
//! ```rust
//! use midgard_rs::Midgard;
//! # tokio_test::block_on(async {
//! let mut midgard = Midgard::new();
//! let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
//! let balance = midgard.get_balance(address, None, None).await.unwrap();
//! assert!(*balance.get_height() > 0);
//! # });
//! ```
//!
//! ## Configuration
//!
//! ```rust
//! use midgard_rs::Midgard;
//! use midgard_rs::Configuration;
//! # tokio_test::block_on(async {
//! let config = Configuration::new("https://gateway.liquify.com/chain/thorchain_midgard/v2/".to_string(), 1000); // base_url, rate_limit_ms
//! let mut midgard = Midgard::with_config(config);
//! let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
//! let balance = midgard.get_balance(address, None, None).await.unwrap(); // address, timestamp, height
//! assert!(*balance.get_height() > 0);
//! # });
//! ```
//!
//! ## Errors
//!
//! Every endpoint returns `Result<_, `[`APIError`]`>`, so the failure is a
//! concrete enum you can match on: [`APIError::HttpStatus`] when the instance
//! answered with a non-success status, [`APIError::ReqwestError`] when the
//! request could not be made or read, [`APIError::SerdeError`] when the body
//! was not the JSON this crate expects, [`APIError::InvalidParameter`] when the
//! arguments were rejected before any request was made,
//! [`APIError::QueryEncode`] when a query string could not be encoded, and
//! [`APIError::ClientBuild`] when the HTTP client could not be constructed. The
//! enum is `#[non_exhaustive]`, so new variants are not a breaking change.
//!
//! ## TLS
//!
//! TLS is `rustls` with the bundled `webpki-roots` store; there is no OpenSSL
//! in the dependency graph. If you need your operating system's trust store
//! instead, depend on `reqwest` with `rustls-tls-native-roots` yourself.
//!
//! ## A note on public instances
//!
//! `midgard.ninerealms.com`, which this crate used as its default through
//! 0.0.5, no longer resolves at all, and `midgard.thorswap.net` answers
//! non-browser clients with a Cloudflare challenge instead of JSON. If the
//! default gateway stops working for you, check the `THORChain` dev docs
//! linked above for the current list and pass it via `Configuration`.
//!

pub(crate) use api::{api_get_action_list, api_get_balance, api_get_borrowers_details, api_get_borrowers_list, api_get_churn_list, api_get_depth_and_price_history, api_get_details_of_pool, api_get_earnings_history, api_get_global_stats, api_get_health_info, api_get_known_pool_list, api_get_liquidity_change_history, api_get_member_details, api_get_member_list, api_get_network_data, api_get_node_list, api_get_pool_list, api_get_savers_details, api_get_savers_units_and_depth_history, api_get_statistics_of_pool, api_get_swaps_history, api_get_thorname_details, api_get_thorname_owner, api_get_thorname_reverse_lookup, api_get_total_value_locked_history};
pub use midgard::*;
pub use types::*;

mod api;
mod midgard;
mod types;

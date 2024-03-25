#![warn(clippy::pedantic, clippy::nursery, clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

//! # Midgard
//! Consumer information relating to swaps, pools, and volume. Midgard returns time-series information regarding the `THORChain` network, such as volume, pool information, users, liquidity providers and more. It also proxies to `THORNode` to reduce burden on the network. Runs on every node.
//! 
//! ## midgard-rs
//! This crate aims to provide fully typed client for the `THORChain` Midgard API. 
//! * By default it references the `https://midgard.thorswap.net/v2/` base url but this can be changed by creating a new `Configuration` object and passing it to the `Midgard::with_config()` method.
//! * The client is rate limited to 1 request per second by default but this can be changed by creating a new `Configuration` object and passing it to the `Midgard::with_config()` method.
//! 
//! ## Basic Usage
//! 
//! ```rust
//! use midgard::Midgard;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let midgard = Midgard::new();
//!     let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
//!     let balance = midgard.get_balance(address, None, None).await.unwrap();
//!     println!("coins: {:?}", balance.get_coins());
//! }
//! ```
//! 
//! ## Configuration
//! 
//! ```rust
//! use midgard::Midgard;
//! use midgard::Configuration;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let config = Configuration::new("https://midgard.ninerealms.com/v2/".to_string(), 1000); // base_url, rate_limit_ms
//!     let midgard = Midgard::with_config(config);
//!     let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
//!     let balance = midgard.get_balance(address, None, None).await.unwrap(); // address, timestamp, height
//!     println!("coins: {:?}", balance.get_coins());
//! }
//! ```
//! 

pub(crate) use api::{api_get_action_list, api_get_balance, api_get_borrowers_details, api_get_borrowers_list, api_get_churn_list, api_get_depth_and_price_history, api_get_details_of_pool, api_get_earnings_history, api_get_global_stats, api_get_health_info, api_get_known_pool_list, api_get_liquidity_change_history, api_get_member_details, api_get_member_list, api_get_network_data, api_get_node_list, api_get_pool_list, api_get_savers_details, api_get_savers_units_and_depth_history, api_get_statistics_of_pool, api_get_swaps_history, api_get_thorname_details, api_get_thorname_owner, api_get_thorname_reverse_lookup, api_get_total_value_locked_history};
pub use midgard::*;
pub use types::*;

mod api;
mod midgard;
mod types;



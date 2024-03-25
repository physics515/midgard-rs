use anyhow::Result;
use chrono::Utc;

use crate::{api_get_depth_and_price_history, api_get_earnings_history, api_get_liquidity_change_history, api_get_savers_units_and_depth_history, api_get_swaps_history, api_get_total_value_locked_history, DepthHistory, EarningsHistory, Interval, LiquidityChangeHistory, Midgard, SaversHistory, SwapHistory, TVLHistory};

impl Midgard {
	/// Returns the asset and rune depths and price. The values report the state at the end of each interval.
	/// History endpoint has two modes:
	/// * With Interval parameter it returns a series of time buckets. From and To dates will be rounded to the Interval boundaries.
	/// * Without Interval parameter a single From..To search is performed with exact timestamps.
	/// * Interval: possible values: 5min, hour, day, week, month, quarter, year.
	/// * count: [1..400]. Defines number of intervals. Don't provide if Interval is missing.
	/// * from/to: optional int, unix second.
	///
	/// Possible usages with interval.
	/// * last 10 days: ?interval=day&count=10
	/// * last 10 days before to: ?interval=day&count=10&to=1608825600
	/// * next 10 days after from: ?interval=day&count=10&from=1606780800
	/// * Days between from and to. From defaults to start of chain, to defaults to now. Only the first 400 intervals are returned: interval=day&from=1606780800&to=1608825600
	///
	/// Pagination is possible with from&count and then using the returned meta.endTime as the From parameter of the next query.
	///
	/// Possible configurations without interval:
	/// * exact search for one time frame: ?from=1606780899&to=1608825600
	/// * one time frame until now: ?from=1606780899
	/// * from chain start until now: no query parameters
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get depth & price history
	/// let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	/// assert!(!depth_history.get_intervals().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `end_time` from the previous response to the next request.
	///
	/// ```
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # std::thread::sleep(std::time::Duration::from_secs(10));
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get depth & price history
	/// let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	/// assert!(!depth_history.get_intervals().is_empty());
	///
	/// // Get the end time
	/// let end_time = depth_history.get_meta().get_end_time().timestamp() as u64;
	///
	/// let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
	///
	/// assert!(!depth_history.get_intervals().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_depth_and_price_history(&mut self, pool: &str, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<DepthHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_depth_and_price_history(self.get_config().get_base_url(), pool, interval, count, to, from).await
	}

	/// Returns earnings data for the specified interval.
	///
	/// History endpoint has two modes:
	/// * With Interval parameter it returns a series of time buckets. From and To dates will be rounded to the Interval boundaries.
	/// * Without Interval parameter a single From..To search is performed with exact timestamps.
	/// * Interval: possible values: 5min, hour, day, week, month, quarter, year.
	/// * count: [1..400]. Defines number of intervals. Don't provide if Interval is missing.
	/// * from/to: optional int, unix second.
	///
	/// Possible usages with interval.
	/// * last 10 days: ?interval=day&count=10
	/// * last 10 days before to: ?interval=day&count=10&to=1608825600
	/// * next 10 days after from: ?interval=day&count=10&from=1606780800
	/// * Days between from and to. From defaults to start of chain, to defaults to now. Only the first 400 intervals are returned: interval=day&from=1606780800&to=1608825600
	///
	/// Pagination is possible with from&count and then using the returned meta.endTime as the From parameter of the next query.
	///
	/// Possible configurations without interval:
	/// * exact search for one time frame: ?from=1606780899&to=1608825600
	/// * one time frame until now: ?from=1606780899
	/// * from chain start until now: no query parameters
	///
	/// # Example
	///
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	///let mut midgard = Midgard::new();
	///
	///// Get depth & price history
	///let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	///assert!(!depth_history.get_intervals().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `end_time` from the previous response to the next request.
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # std::thread::sleep(std::time::Duration::from_secs(10));
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get depth & price history
	/// let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	/// assert!(!depth_history.get_intervals().is_empty());
	///
	/// // Get the end time
	/// let end_time = depth_history.get_meta().get_end_time().timestamp() as u64;
	///
	/// let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
	///
	/// assert!(!depth_history.get_intervals().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_earnings_history(&mut self, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<EarningsHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_earnings_history(self.get_config().get_base_url(), interval, count, to, from).await
	}

	/// Returns withdrawals and deposits for given time interval. If pool is not specified returns for all pools
	///
	/// History endpoint has two modes:
	/// * With Interval parameter it returns a series of time buckets. From and To dates will be rounded to the Interval boundaries.
	/// * Without Interval parameter a single From..To search is performed with exact timestamps.
	/// * Interval: possible values: 5min, hour, day, week, month, quarter, year.
	/// * count: [1..400]. Defines number of intervals. Don't provide if Interval is missing.
	/// * from/to: optional int, unix second.
	///
	/// Possible usages with interval.
	/// * last 10 days: ?interval=day&count=10
	/// * last 10 days before to: ?interval=day&count=10&to=1608825600
	/// * next 10 days after from: ?interval=day&count=10&from=1606780800
	/// * Days between from and to. From defaults to start of chain, to defaults to now. Only the first 400 intervals are returned: interval=day&from=1606780800&to=1608825600
	///
	/// Pagination is possible with from&count and then using the returned meta.endTime as the From parameter of the next query.
	///
	/// Possible configurations without interval:
	/// * exact search for one time frame: ?from=1606780899&to=1608825600
	/// * one time frame until now: ?from=1606780899
	/// * from chain start until now: no query parameters
	///
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get liquidity change history
	/// let liquidity_change_history = midgard.get_liquidity_change_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	/// assert!(!liquidity_change_history.get_intervals().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `end_time` from the previous response to the next request.
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # std::thread::sleep(std::time::Duration::from_secs(10));
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get liquidity change history
	/// let liquidity_change_history = midgard.get_liquidity_change_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	/// assert!(!liquidity_change_history.get_intervals().is_empty());
	///
	/// // Get the end time
	/// let end_time = liquidity_change_history.get_meta().get_end_time().timestamp() as u64;
	///
	/// let liquidity_change_history = midgard.get_liquidity_change_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
	///
	/// assert!(!liquidity_change_history.get_intervals().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_liquidity_change_history(&mut self, pool: &str, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<LiquidityChangeHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_liquidity_change_history(self.get_config().get_base_url(), pool, interval, count, to, from).await
	}

	/// Returns savers depths and units. The values report the state at the end of each interval.
	///
	/// History endpoint has two modes:
	/// * With Interval parameter it returns a series of time buckets. From and To dates will be rounded to the Interval boundaries.
	/// * Without Interval parameter a single From..To search is performed with exact timestamps.
	/// * Interval: possible values: 5min, hour, day, week, month, quarter, year.
	/// * count: [1..400]. Defines number of intervals. Don't provide if Interval is missing.
	/// * from/to: optional int, unix second.
	///
	/// Possible usages with interval.
	/// * last 10 days: ?interval=day&count=10
	/// * last 10 days before to: ?interval=day&count=10&to=1608825600
	/// * next 10 days after from: ?interval=day&count=10&from=1606780800
	/// * Days between from and to. From defaults to start of chain, to defaults to now. Only the first 400 intervals are returned: interval=day&from=1606780800&to=1608825600
	///
	/// Pagination is possible with from&count and then using the returned meta.endTime as the From parameter of the next query.
	///
	/// Possible configurations without interval:
	/// * exact search for one time frame: ?from=1606780899&to=1608825600
	/// * one time frame until now: ?from=1606780899
	/// * from chain start until now: no query parameters
	///
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get savers units and depth history
	/// let savers_history = midgard.get_savers_units_and_depth_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	///
	/// assert!(!savers_history.get_intervals().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `end_time` from the previous response to the next request.
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # std::thread::sleep(std::time::Duration::from_secs(10));
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	///
	/// // Get savers units and depth history
	/// let savers_history = midgard.get_savers_units_and_depth_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();
	/// assert!(!savers_history.get_intervals().is_empty());
	///
	/// // Get the end time
	/// let end_time = savers_history.get_meta().get_end_time().timestamp() as u64;
	/// let savers_history = midgard.get_savers_units_and_depth_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
	/// assert!(!savers_history.get_intervals().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_savers_units_and_depth_history(&mut self, pool: &str, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<SaversHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_savers_units_and_depth_history(self.get_config().get_base_url(), pool, interval, count, to, from).await
	}

	/// Returns swap count, volume, fees, slip in specified interval. If pool is not specified returns for all pools
	///
	/// History endpoint has two modes:
	/// * With Interval parameter it returns a series of time buckets. From and To dates will be rounded to the Interval boundaries.
	/// * Without Interval parameter a single From..To search is performed with exact timestamps.
	/// * Interval: possible values: 5min, hour, day, week, month, quarter, year.
	/// * count: [1..400]. Defines number of intervals. Don't provide if Interval is missing.
	/// * from/to: optional int, unix second.
	///
	/// Possible usages with interval.
	/// * last 10 days: ?interval=day&count=10
	/// * last 10 days before to: ?interval=day&count=10&to=1608825600
	/// * next 10 days after from: ?interval=day&count=10&from=1606780800
	/// * Days between from and to. From defaults to start of chain, to defaults to now. Only the first 400 intervals are returned: interval=day&from=1606780800&to=1608825600
	///
	/// Pagination is possible with from&count and then using the returned meta.endTime as the From parameter of the next query.
	///
	/// Possible configurations without interval:
	/// * exact search for one time frame: ?from=1606780899&to=1608825600
	/// * one time frame until now: ?from=1606780899
	/// * from chain start until now: no query parameters
	///
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	/// // Get swaps history
	/// let swaps_history = midgard.get_swaps_history(None, Some(Interval::Day), Some(10), None, None).await.unwrap();
	/// assert!(!swaps_history.get_intervals().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `end_time` from the previous response to the next request.
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # std::thread::sleep(std::time::Duration::from_secs(10));
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	/// // Get swaps history
	/// let swaps_history = midgard.get_swaps_history(None, Some(Interval::Day), Some(10), None, None).await.unwrap();
	/// assert!(!swaps_history.get_intervals().is_empty());
	///
	/// // Get the end time
	/// let end_time = swaps_history.get_meta().get_end_time().timestamp() as u64;
	/// let swaps_history = midgard.get_swaps_history(None, Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
	/// assert!(!swaps_history.get_intervals().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_swaps_history(&mut self, pool: Option<&str>, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<SwapHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		api_get_swaps_history(self.get_config().get_base_url(), pool, interval, count, to, from).await
	}

	/// Returns total pool depths, total bonds, and total value locked in specified interval.
	///
	/// Total Value Locked = Total Bonds + 2 * Total Pool Depths
	///
	/// History endpoint has two modes:
	/// * With Interval parameter it returns a series of time buckets. From and To dates will be rounded to the Interval boundaries.
	/// * Without Interval parameter a single From..To search is performed with exact timestamps.
	/// * Interval: possible values: 5min, hour, day, week, month, quarter, year.
	/// * count: [1..400]. Defines number of intervals. Don't provide if Interval is missing.
	/// * from/to: optional int, unix second.
	///
	/// Possible usages with interval.
	/// * last 10 days: ?interval=day&count=10
	/// * last 10 days before to: ?interval=day&count=10&to=1608825600
	/// * next 10 days after from: ?interval=day&count=10&from=1606780800
	/// * Days between from and to. From defaults to start of chain, to defaults to now. Only the first 400 intervals are returned: interval=day&from=1606780800&to=1608825600
	///
	/// Pagination is possible with from&count and then using the returned meta.endTime as the From parameter of the next query.
	///
	/// Possible configurations without interval:
	/// * exact search for one time frame: ?from=1606780899&to=1608825600
	/// * one time frame until now: ?from=1606780899
	/// * from chain start until now: no query parameters
	///
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	/// // Get total value locked history
	/// let tvl_history = midgard.get_total_value_locked_history(Some(Interval::Day), Some(10), None, None).await.unwrap();
	/// assert!(!tvl_history.get_intervals().is_empty());
	/// # });
	/// ```
	///
	/// To get paginated responses, you can pass the `end_time` from the previous response to the next request.
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::Interval;
	///
	/// # std::thread::sleep(std::time::Duration::from_secs(10));
	/// # tokio_test::block_on(async {
	/// // Create a new instance of Midgard
	/// let mut midgard = Midgard::new();
	/// // Get total value locked history
	/// let tvl_history = midgard.get_total_value_locked_history(Some(Interval::Day), Some(10), None, None).await.unwrap();
	/// assert!(!tvl_history.get_intervals().is_empty());
	///
	/// // Get the end time
	/// let end_time = tvl_history.get_meta().get_end_time().timestamp() as u64;
	/// let tvl_history = midgard.get_total_value_locked_history(Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
	/// assert!(!tvl_history.get_intervals().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_total_value_locked_history(&mut self, interval: Option<Interval>, count: Option<usize>, to: Option<u64>, from: Option<u64>) -> Result<TVLHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_total_value_locked_history(self.get_config().get_base_url(), interval, count, to, from).await
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[tokio::test]
	async fn test_get_depth_and_price_history() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get depth & price history
		let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("depth history: {}", json!(depth_history));

		assert!(!depth_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_depth_and_price_history_pagination() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get depth & price history
		let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("depth history: {}", json!(depth_history));

		assert!(!depth_history.get_intervals().is_empty());

		// Get the end time
		let end_time = depth_history.get_meta().get_end_time().timestamp() as u64;

		let depth_history = midgard.get_depth_and_price_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();

		println!("depth history: {}", json!(depth_history));

		assert!(!depth_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_earnings_history() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get earnings history
		let earnings_history = midgard.get_earnings_history(Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("earnings history: {}", json!(earnings_history));

		assert!(!earnings_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_earnings_history_pagination() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get earnings history
		let earnings_history = midgard.get_earnings_history(Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("earnings history: {}", json!(earnings_history));

		assert!(!earnings_history.get_intervals().is_empty());

		// Get the end time
		let end_time = earnings_history.get_meta().get_end_time().timestamp() as u64;

		let earnings_history = midgard.get_earnings_history(Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();

		println!("earnings history: {}", json!(earnings_history));

		assert!(!earnings_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_liquidity_change_history() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get liquidity change history
		let liquidity_change_history = midgard.get_liquidity_change_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("liquidity change history: {}", json!(liquidity_change_history));

		assert!(!liquidity_change_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_liquidity_change_history_pagination() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get liquidity change history
		let liquidity_change_history = midgard.get_liquidity_change_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("liquidity change history: {}", json!(liquidity_change_history));

		assert!(!liquidity_change_history.get_intervals().is_empty());

		// Get the end time
		let end_time = liquidity_change_history.get_meta().get_end_time().timestamp() as u64;

		let liquidity_change_history = midgard.get_liquidity_change_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();

		println!("liquidity change history: {}", json!(liquidity_change_history));

		assert!(!liquidity_change_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_savers_units_and_depth_history() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get savers units and depth history
		let savers_history = midgard.get_savers_units_and_depth_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("savers history: {}", json!(savers_history));

		assert!(!savers_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_savers_units_and_depth_history_pagination() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();

		// Get savers units and depth history
		let savers_history = midgard.get_savers_units_and_depth_history("BTC.BTC", Some(Interval::Day), Some(10), None, None).await.unwrap();

		println!("savers history: {}", json!(savers_history));

		assert!(!savers_history.get_intervals().is_empty());

		// Get the end time
		let end_time = savers_history.get_meta().get_end_time().timestamp() as u64;

		let savers_history = midgard.get_savers_units_and_depth_history("BTC.BTC", Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();

		println!("savers history: {}", json!(savers_history));

		assert!(!savers_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_swaps_history() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();
		// Get swaps history
		let swaps_history = midgard.get_swaps_history(None, Some(Interval::Day), Some(10), None, None).await.unwrap();
		println!("swaps history: {}", json!(swaps_history));
		assert!(!swaps_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_swaps_history_pagination() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();
		// Get swaps history
		let swaps_history = midgard.get_swaps_history(None, Some(Interval::Day), Some(10), None, None).await.unwrap();
		println!("swaps history: {}", json!(swaps_history));
		assert!(!swaps_history.get_intervals().is_empty());

		// Get the end time
		let end_time = swaps_history.get_meta().get_end_time().timestamp() as u64;
		let swaps_history = midgard.get_swaps_history(None, Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
		println!("swaps history: {}", json!(swaps_history));
		assert!(!swaps_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_total_value_locked_history() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();
		// Get total value locked history
		let tvl_history = midgard.get_total_value_locked_history(Some(Interval::Day), Some(10), None, None).await.unwrap();
		println!("tvl history: {}", json!(tvl_history));
		assert!(!tvl_history.get_intervals().is_empty());
	}

	#[tokio::test]
	async fn test_get_total_value_locked_history_pagination() {
		// Create a new instance of Midgard
		let mut midgard = Midgard::new();
		// Get total value locked history
		let tvl_history = midgard.get_total_value_locked_history(Some(Interval::Day), Some(10), None, None).await.unwrap();
		println!("tvl history: {}", json!(tvl_history));
		assert!(!tvl_history.get_intervals().is_empty());

		// Get the end time
		let end_time = tvl_history.get_meta().get_end_time().timestamp() as u64;
		let tvl_history = midgard.get_total_value_locked_history(Some(Interval::Day), Some(10), None, Some(end_time)).await.unwrap();
		println!("tvl history: {}", json!(tvl_history));
		assert!(!tvl_history.get_intervals().is_empty());
	}
}

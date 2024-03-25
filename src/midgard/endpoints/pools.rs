use anyhow::Result;
use chrono::Utc;

use crate::{api_get_details_of_pool, api_get_pool_list, api_get_known_pool_list, api_get_statistics_of_pool, Midgard, KnownPoolList, Pool, PoolList, PoolStatus, PoolStatistics, TimePeriod};

impl Midgard {
	/// Returns an array containing details for a set of pools.
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::PoolStatus;
	/// use midgard_rs::TimePeriod;
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	///
	/// let pool_list = midgard.get_pool_list(None, None).await.unwrap();
	/// assert!(!pool_list.get_pools().is_empty());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_pool_list(&mut self, status: Option<PoolStatus>, period: Option<TimePeriod>) -> Result<PoolList> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_pool_list(self.get_config().get_base_url(), status, period).await
	}

	/// Returns details of the pool: depths, price, 24h volume, APY.
	/// # Example
	/// ```rust
	/// use midgard_rs::Midgard;
	/// use midgard_rs::TimePeriod;
	/// use rand::prelude::*;
	/// # tokio_test::block_on(async {
	/// let mut midgard = Midgard::new();
	///
        /// // Get a random pool
	/// let pool_list = midgard.get_pool_list(None, None).await.unwrap();
	/// let random_usize = thread_rng().gen_range(0..pool_list.get_pools().len());
	/// let pool = pool_list.get_pools()[random_usize].clone();
	/// let pool = pool.get_asset().to_string();
	///
        /// // Get details of the pool
	/// let details = midgard.get_details_of_pool(&pool, None).await.unwrap();
	/// assert!(!details.get_annual_percentage_rate().is_zero());
	/// # });
	/// ```
        /// 
        /// # Errors
        /// todo
	pub async fn get_details_of_pool(&mut self, pool: &str, period: Option<TimePeriod>) -> Result<Pool> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_details_of_pool(self.get_config().get_base_url(), pool, period).await
	}

        /// Returns an object with known pools and their statuses.
        /// # Example
        /// ```rust
        /// use midgard_rs::Midgard;
        /// use midgard_rs::PoolStatus;
        /// # tokio_test::block_on(async {
        /// let mut midgard = Midgard::new();
        /// 
        /// let known_pool_list = midgard.get_known_pool_list().await.unwrap();
        /// assert!(!known_pool_list.into_iter().collect::<Vec<(String, PoolStatus)>>().is_empty());
        /// # });
        /// ```
        /// 
        /// # Errors
        /// todo
        pub async fn get_known_pool_list(&mut self) -> Result<KnownPoolList> {
                // Wait for rate limit timer
                self.sleep_until_ok_to_call().await;

                self.set_last_call(Utc::now());
                api_get_known_pool_list(self.get_config().get_base_url()).await
        }

        /// Statistics about the pool. The description of the fields have pointers about the corresponding v2/history location. Visit the history endpoint for drilldowns.
        /// # Example
        /// ```rust
        /// use midgard_rs::Midgard;
        /// use midgard_rs::TimePeriod;
        /// use rand::prelude::*;
        /// # tokio_test::block_on(async {
        /// let mut midgard = Midgard::new();
        /// 
        /// // Get a random pool
        /// let pool_list = midgard.get_pool_list(None, None).await.unwrap();
        /// let random_usize = thread_rng().gen_range(0..pool_list.get_pools().len());
        /// let pool = pool_list.get_pools()[random_usize].clone();
        /// let pool = pool.get_asset().to_string();
        /// 
        /// // Get statistics of the pool
        /// let pool_statistics = midgard.get_statistics_of_pool(&pool, None).await.unwrap();
        /// assert!(!pool_statistics.get_asset().is_empty());
        /// # });
        /// ```
        /// 
        /// # Errors
        /// todo
        pub async fn get_statistics_of_pool(&mut self, pool: &str, period: Option<TimePeriod>) -> Result<PoolStatistics> {
                // Wait for rate limit timer
                self.sleep_until_ok_to_call().await;

                self.set_last_call(Utc::now());
                api_get_statistics_of_pool(self.get_config().get_base_url(), pool, period).await
        }

}

#[cfg(test)]
mod tests {
	use rand::prelude::*;
	use serde_json::json;

	use super::*;
	use crate::api_get_pool_list;

	#[tokio::test]
	async fn test_get_pool_list() {
		let mut midgard = Midgard::new();

		let pool_list = midgard.get_pool_list(None, None).await.unwrap();
		println!("{}", json!(pool_list.get_pools()));
		assert!(!pool_list.get_pools().is_empty());
	}

	#[tokio::test]
	async fn test_get_simplified_assets() {
		let mut midgard = Midgard::new();

		let pool_list = midgard.get_pool_list(None, None).await.unwrap();
		println!("{}", json!(pool_list.get_simplified_assets()));
		assert!(!pool_list.get_simplified_assets().is_empty());
	}

	#[tokio::test]
	async fn test_get_pool_list_with_status() {
		let mut midgard = Midgard::new();

		let status = Some(PoolStatus::Available);
		let period = None;

		let pool_list = midgard.get_pool_list(status, period).await.unwrap();
		println!("{}", json!(pool_list.get_pools()));
		assert!(!pool_list.get_pools().is_empty());
	}

	#[tokio::test]
	async fn test_get_pool_list_with_period() {
		let mut midgard = Midgard::new();

		let status = None;
		let period = Some(TimePeriod::SevenDays);

		let pool_list = midgard.get_pool_list(status, period).await.unwrap();
		println!("{}", json!(pool_list.get_pools()));
		assert!(!pool_list.get_pools().is_empty());
	}

	#[tokio::test]
	async fn test_get_pool_list_with_status_and_period() {
		let mut midgard = Midgard::new();

		let status = Some(PoolStatus::Available);
		let period = Some(TimePeriod::FourteenDays);

		let pool_list = midgard.get_pool_list(status, period).await.unwrap();
		println!("{}", json!(pool_list.get_pools()));
		assert!(!pool_list.get_pools().is_empty());
	}

	#[tokio::test]
	async fn test_get_details_of_pool() {
		let mut midgard = Midgard::new();

		let pool_list = midgard.get_pool_list(None, None).await.unwrap();
		let random_usize = thread_rng().gen_range(0..pool_list.get_pools().len());
		let pool = pool_list.get_pools()[random_usize].clone();
		let pool = pool.get_asset().to_string();
		println!("pool: {}", &pool);

		let details = midgard.get_details_of_pool(&pool, None).await.unwrap();
		println!("{}", json!(details));
		assert!(!details.get_annual_percentage_rate().is_zero());
	}

        #[tokio::test]
        async fn test_get_known_pool_list() {
                let mut midgard = Midgard::new();

                let known_pool_list = midgard.get_known_pool_list().await.unwrap();
                println!("{}", json!(known_pool_list));
                assert!(!known_pool_list.into_iter().collect::<Vec<(String, PoolStatus)>>().is_empty());
        }

        #[tokio::test]
        async fn test_get_statistics_of_pool() {
                let mut midgard = Midgard::new();

                // Get a random pool
                let pool_list = midgard.get_pool_list(None, None).await.unwrap();
                let random_usize = thread_rng().gen_range(0..pool_list.get_pools().len());
                let pool = pool_list.get_pools()[random_usize].clone();
                let pool = pool.get_asset().to_string();
                println!("pool: {}", &pool);

                // Get statistics of the pool
                let pool_statistics = midgard.get_statistics_of_pool(&pool, None).await.unwrap();
                println!("{}", json!(pool_statistics));
                assert!(!pool_statistics.get_asset().is_empty());
        }
}

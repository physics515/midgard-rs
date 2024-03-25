use anyhow::Result;
use chrono::Utc;

use crate::Midgard;
use crate::{api_get_savers_details, SaversDetails};

impl Midgard {
        ///Returns an array of statistics for all the savers associated with a given member address. Query can also be multiple addresses should be seperated by comma (',').
        /// # Example
        /// ```rust
        /// use midgard_rs::Midgard;
        /// use midgard_rs::SaversDetails;
        /// # tokio_test::block_on(async {
        /// let mut midgard = Midgard::new();
        /// let address = vec![
        ///         "bnb1jxfh2g85q3v0tdq56fnevx6xcxtcnhtsmcu64m".to_string(),
        ///         "bc1qcxssye4j6730h7ehgega3gyykkuwgdgmmpu62n".to_string(),
        /// ];
        /// 
        /// let savers_details = midgard.get_savers_details(&address).await.unwrap();
        /// assert!(!savers_details.get_pools().is_empty());
        /// # });
        /// ```
        /// 
        /// # Errors
        /// todo
        pub async fn get_savers_details(&mut self, address: &[String]) -> Result<SaversDetails> {
                // Wait for rate limit timer
                self.sleep_until_ok_to_call().await;

                self.set_last_call(Utc::now());
                api_get_savers_details(self.get_config().get_base_url(), address).await
        }
}

#[cfg(test)]
mod tests {
        use rand::prelude::*;
        use serde_json::json;

        use super::*;
        use crate::api_get_savers_details;

        #[tokio::test]
        async fn test_get_savers_details() {
                let mut midgard = Midgard::new();
                let address = vec![
                        "bnb1jxfh2g85q3v0tdq56fnevx6xcxtcnhtsmcu64m".to_string(),
                        "bc1qcxssye4j6730h7ehgega3gyykkuwgdgmmpu62n".to_string(),
                ];

                let savers_details = midgard.get_savers_details(&address).await.unwrap();
                println!("{}", json!(savers_details));
                assert!(!savers_details.get_pools().is_empty());
        }
}
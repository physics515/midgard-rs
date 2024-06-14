use anyhow::Result;
use chrono::Utc;

use crate::Midgard;
use crate::{api_get_thorname_details, api_get_thorname_owner, api_get_thorname_reverse_lookup, ThornameDetails, ThornameOwner, ThornameReverseLookup};

impl Midgard {
        /// Returns an array of chains and their addresses associated with the given `THORName`.
        /// # Example
        /// ```rust
        /// use midgard_rs::Midgard;
        /// 
        /// # tokio_test::block_on(async {
        /// let mut midgard = Midgard::new();
        /// let name = "thorchain";
        /// 
        /// let thorname_details = midgard.get_thorname_details(&name).await.unwrap();
        /// assert!(!thorname_details.get_owner().is_empty());
        /// # });
        /// ```
        /// 
        /// # Errors
        /// todo
        pub async fn get_thorname_details(&mut self, name: &str) -> Result<ThornameDetails> {
                // Wait for rate limit timer
                self.sleep_until_ok_to_call().await;

                self.set_last_call(Utc::now());
                api_get_thorname_details(self.get_config().get_base_url(), name).await
        }

        /// Returns an array of `THORNames` owned by the address. The address is not necessarily an associated address for those thornames.
        /// # Example
        /// ```rust
        /// use midgard_rs::Midgard;
        /// 
        /// # tokio_test::block_on(async {
        /// let mut midgard = Midgard::new();
        /// let address = "thor18w0hsdru75ug0x4uvamgjn6ghlu43mr4dcypq9";
        /// let thorname_owner = midgard.get_thorname_owner(&address).await.unwrap();
        /// assert!(!thorname_owner.get_thorname_owner().is_empty());
        /// # });
        /// ```
        /// 
        /// # Errors
        /// todo
        pub async fn get_thorname_owner(&mut self, address: &str) -> Result<ThornameOwner> {
                // Wait for rate limit timer
                self.sleep_until_ok_to_call().await;

                self.set_last_call(Utc::now());
                api_get_thorname_owner(self.get_config().get_base_url(), address).await
        }

        /// Returns an array of `THORNames` associated with the given address
        /// # Example
        /// ```rust
        /// use midgard_rs::Midgard;
        /// 
        /// # tokio_test::block_on(async {
        /// let mut midgard = Midgard::new();
        /// let address = "thor18w0hsdru75ug0x4uvamgjn6ghlu43mr4dcypq9";
        /// let thorname_owner = midgard.get_thorname_owner(&address).await.unwrap();
        /// assert!(!thorname_owner.get_thorname_owner().is_empty());
        /// # });
        /// ```
        /// # Errors
        /// todo
        pub async fn get_thorname_reverse_lookup(&mut self, address: &str) -> Result<ThornameReverseLookup> {
                // Wait for rate limit timer
                self.sleep_until_ok_to_call().await;

                self.set_last_call(Utc::now());
                api_get_thorname_reverse_lookup(self.get_config().get_base_url(), address).await
        }
}

#[cfg(test)]
mod tests {
        use serde_json::json;

        use super::*;

        #[tokio::test]
        async fn test_get_thorname_details() {
                let mut midgard = Midgard::new();
                let name = "thorchain";

                let thorname_details = midgard.get_thorname_details(&name).await.unwrap();
                println!("{}", json!(thorname_details));
                assert!(!thorname_details.get_owner().is_empty());
        }

        #[tokio::test]
        async fn test_get_thorname_owner() {
                let mut midgard = Midgard::new();
                let address = "thor18w0hsdru75ug0x4uvamgjn6ghlu43mr4dcypq9";

                let thorname_owner = midgard.get_thorname_owner(&address).await.unwrap();
                println!("{}", json!(thorname_owner));
                assert!(!thorname_owner.get_thorname_owner().is_empty());
        }

        #[tokio::test]
        async fn test_get_thorname_reverse_lookup() {
                let mut midgard = Midgard::new();
                let address = "thor18w0hsdru75ug0x4uvamgjn6ghlu43mr4dcypq9";

                let thorname_reverse_lookup = midgard.get_thorname_reverse_lookup(&address).await.unwrap();
                println!("{}", json!(thorname_reverse_lookup));
                assert!(!thorname_reverse_lookup.get_thorname_reverse_lookup().is_empty());
        }
}
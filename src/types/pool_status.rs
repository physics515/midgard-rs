use std::fmt::Display;

use serde::{Deserialize, Serialize};

/*

*** PoolStatus Options ***
available
staged
suspended

*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PoolStatus {
	#[serde(rename = "available")]
	Available,
	#[serde(rename = "staged")]
	Staged,
	#[serde(rename = "suspended")]
	Suspended,
}

impl Display for PoolStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Available => write!(f, "available"),
			Self::Staged => write!(f, "staged"),
			Self::Suspended => write!(f, "suspended"),
		}
	}
}

impl From<&str> for PoolStatus {
	fn from(s: &str) -> Self {
		match s {
			"available" => Self::Available,
			"staged" => Self::Staged,
			_ => Self::Suspended,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pool_status() {
		let status = PoolStatus::Available;
		assert_eq!(status.to_string(), "available");
		let status = PoolStatus::Staged;
		assert_eq!(status.to_string(), "staged");
		let status = PoolStatus::Suspended;
		assert_eq!(status.to_string(), "suspended");
	}

	#[test]
	fn test_pool_status_from_str() {
		let status = "available";
		assert_eq!(PoolStatus::from(status), PoolStatus::Available);
		let status = "staged";
		assert_eq!(PoolStatus::from(status), PoolStatus::Staged);
		let status = "suspended";
		assert_eq!(PoolStatus::from(status), PoolStatus::Suspended);
		let status = "invalid";
		assert_eq!(PoolStatus::from(status), PoolStatus::Suspended);
	}

	#[test]
	fn test_pool_status_serde() {
		let status = PoolStatus::Available;
		let s = serde_json::to_string(&status).unwrap();
		assert_eq!(s, "\"available\"");
		let status = PoolStatus::Suspended;
		let s = serde_json::to_string(&status).unwrap();
		assert_eq!(s, "\"suspended\"");
		let status = PoolStatus::Staged;
		let s = serde_json::to_string(&status).unwrap();
		assert_eq!(s, "\"staged\"");
	}
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/*

*** TimePeriod Options ***
OneHour
TwentyFourHours
SevenDays
FourteenDays
ThirtyDays
NinetyDays
OneHundredDays
OneHundredEightyDays
ThreeHundredSixtyFiveDays
All

*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TimePeriod {
	#[serde(rename = "1h")]
	OneHour,
	#[serde(rename = "24h")]
	TwentyFourHours,
	#[serde(rename = "7d")]
	SevenDays,
	#[serde(rename = "14d")]
	FourteenDays,
	#[serde(rename = "30d")]
	ThirtyDays,
	#[serde(rename = "90d")]
	NinetyDays,
	#[serde(rename = "100d")]
	OneHundredDays,
	#[serde(rename = "180d")]
	OneHundredEightyDays,
	#[serde(rename = "365d")]
	ThreeHundredSixtyFiveDays,
	#[serde(rename = "all")]
	All,
}

impl Default for TimePeriod {
	fn default() -> Self {
		Self::FourteenDays
	}
}

impl Display for TimePeriod {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::OneHour => write!(f, "1h"),
			Self::TwentyFourHours => write!(f, "24h"),
			Self::SevenDays => write!(f, "7d"),
			Self::FourteenDays => write!(f, "14d"),
			Self::ThirtyDays => write!(f, "30d"),
			Self::NinetyDays => write!(f, "90d"),
			Self::OneHundredDays => write!(f, "100d"),
			Self::OneHundredEightyDays => write!(f, "180d"),
			Self::ThreeHundredSixtyFiveDays => write!(f, "365d"),
			Self::All => write!(f, "all"),
		}
	}
}

impl From<&str> for TimePeriod {
	fn from(s: &str) -> Self {
		match s {
			"1h" => Self::OneHour,
			"24h" => Self::TwentyFourHours,
			"7d" => Self::SevenDays,
			"14d" => Self::FourteenDays,
			"30d" => Self::ThirtyDays,
			"90d" => Self::NinetyDays,
			"100d" => Self::OneHundredDays,
			"180d" => Self::OneHundredEightyDays,
			"365d" => Self::ThreeHundredSixtyFiveDays,
			_ => Self::All,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_time_period() {
		let period = TimePeriod::OneHour;
		assert_eq!(period.to_string(), "1h");
		let period = TimePeriod::TwentyFourHours;
		assert_eq!(period.to_string(), "24h");
		let period = TimePeriod::SevenDays;
		assert_eq!(period.to_string(), "7d");
		let period = TimePeriod::FourteenDays;
		assert_eq!(period.to_string(), "14d");
		let period = TimePeriod::ThirtyDays;
		assert_eq!(period.to_string(), "30d");
		let period = TimePeriod::NinetyDays;
		assert_eq!(period.to_string(), "90d");
		let period = TimePeriod::OneHundredDays;
		assert_eq!(period.to_string(), "100d");
		let period = TimePeriod::OneHundredEightyDays;
		assert_eq!(period.to_string(), "180d");
		let period = TimePeriod::ThreeHundredSixtyFiveDays;
		assert_eq!(period.to_string(), "365d");
		let period = TimePeriod::All;
		assert_eq!(period.to_string(), "all");
	}

	#[test]
	fn test_time_period_from_str() {
		let period = "1h";
		assert_eq!(TimePeriod::from(period), TimePeriod::OneHour);
		let period = "24h";
		assert_eq!(TimePeriod::from(period), TimePeriod::TwentyFourHours);
		let period = "7d";
		assert_eq!(TimePeriod::from(period), TimePeriod::SevenDays);
		let period = "14d";
		assert_eq!(TimePeriod::from(period), TimePeriod::FourteenDays);
		let period = "30d";
		assert_eq!(TimePeriod::from(period), TimePeriod::ThirtyDays);
		let period = "90d";
		assert_eq!(TimePeriod::from(period), TimePeriod::NinetyDays);
		let period = "100d";
		assert_eq!(TimePeriod::from(period), TimePeriod::OneHundredDays);
		let period = "180d";
		assert_eq!(TimePeriod::from(period), TimePeriod::OneHundredEightyDays);
		let period = "365d";
		assert_eq!(TimePeriod::from(period), TimePeriod::ThreeHundredSixtyFiveDays);
		let period = "invalid";
		assert_eq!(TimePeriod::from(period), TimePeriod::All);
	}

	#[test]
	fn test_time_period_serde() {
		let period = TimePeriod::OneHour;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"1h\"");
		let period = TimePeriod::TwentyFourHours;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"24h\"");
		let period = TimePeriod::SevenDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"7d\"");
		let period = TimePeriod::FourteenDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"14d\"");
		let period = TimePeriod::ThirtyDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"30d\"");
		let period = TimePeriod::NinetyDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"90d\"");
		let period = TimePeriod::OneHundredDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"100d\"");
		let period = TimePeriod::OneHundredEightyDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"180d\"");
		let period = TimePeriod::ThreeHundredSixtyFiveDays;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"365d\"");
		let period = TimePeriod::All;
		let s = serde_json::to_string(&period).unwrap();
		assert_eq!(s, "\"all\"");
	}
}

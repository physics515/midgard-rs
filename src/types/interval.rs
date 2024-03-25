use std::fmt::Display;

use serde::{Deserialize, Serialize};

/*

*** Interval Options ***
5min
hour
day
week
month
quarter
year

*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Interval {
	#[serde(rename = "5min")]
	FiveMinutes,
	#[serde(rename = "hour")]
	Hour,
	#[serde(rename = "day")]
	Day,
	#[serde(rename = "week")]
	Week,
	#[serde(rename = "month")]
	Month,
	#[serde(rename = "quarter")]
	Quarter,
	#[serde(rename = "year")]
	Year,
}

impl Default for Interval {
	fn default() -> Self {
		Self::Day
	}
}

impl Display for Interval {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::FiveMinutes => write!(f, "5min"),
			Self::Hour => write!(f, "hour"),
			Self::Day => write!(f, "day"),
			Self::Week => write!(f, "week"),
			Self::Month => write!(f, "month"),
			Self::Quarter => write!(f, "quarter"),
			Self::Year => write!(f, "year"),
		}
	}
}

impl From<&str> for Interval {
	fn from(s: &str) -> Self {
		match s {
			"5min" => Self::FiveMinutes,
			"hour" => Self::Hour,
			"week" => Self::Week,
			"month" => Self::Month,
			"quarter" => Self::Quarter,
			"year" => Self::Year,
			_ => Self::Day,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_interval_default() {
		assert_eq!(Interval::default(), Interval::Day);
	}

	#[test]
	fn test_interval_display() {
		assert_eq!(Interval::FiveMinutes.to_string(), "5min");
		assert_eq!(Interval::Hour.to_string(), "hour");
		assert_eq!(Interval::Day.to_string(), "day");
		assert_eq!(Interval::Week.to_string(), "week");
		assert_eq!(Interval::Month.to_string(), "month");
		assert_eq!(Interval::Quarter.to_string(), "quarter");
		assert_eq!(Interval::Year.to_string(), "year");
	}

	#[test]
	fn test_interval_from() {
		assert_eq!(Interval::from("5min"), Interval::FiveMinutes);
		assert_eq!(Interval::from("hour"), Interval::Hour);
		assert_eq!(Interval::from("day"), Interval::Day);
		assert_eq!(Interval::from("week"), Interval::Week);
		assert_eq!(Interval::from("month"), Interval::Month);
		assert_eq!(Interval::from("quarter"), Interval::Quarter);
		assert_eq!(Interval::from("year"), Interval::Year);
		assert_eq!(Interval::from("invalid"), Interval::Day);
	}
}

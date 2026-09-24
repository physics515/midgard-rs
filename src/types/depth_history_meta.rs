use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** DepthHistoryMeta Scheme ***

{
		"endAssetDepth": "132260464553",
		"endLPUnits": "384914867482691",
		"endMemberCount": "3639",
		"endRuneDepth": "1039401868675364",
		"endSynthUnits": "239652373224078",
		"endTime": "1710187200",
		"luviIncrease": "1.0048653707710198",
		"priceShiftLoss": "0.9999706799363",
		"startAssetDepth": "132120136671",
		"startLPUnits": "384914706198095",
		"startMemberCount": "3640",
		"startRuneDepth": "1054323593024920",
		"startSynthUnits": "246844795355131",
		"startTime": "1710169200"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct DepthHistoryMeta {
	#[serde(rename = "endAssetDepth", deserialize_with = "deserialize_number_from_string")]
	end_asset_depth: u64,

	#[serde(rename = "endLPUnits", deserialize_with = "deserialize_number_from_string")]
	end_lp_units: u64,

	#[serde(rename = "endMemberCount", deserialize_with = "deserialize_number_from_string")]
	end_member_count: u64,

	#[serde(rename = "endRuneDepth", deserialize_with = "deserialize_number_from_string")]
	end_rune_depth: u64,

	/// Signed: these go negative on pools whose synths have been burnt
	/// below the recorded baseline.
	#[serde(rename = "endSynthUnits", deserialize_with = "deserialize_number_from_string")]
	end_synth_units: i64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	/// `None` when Midgard reports `"NaN"`, i.e. LUVI is undefined over the
	/// requested window.
	#[serde(rename = "luviIncrease", with = "crate::types::decimal_nan::optional")]
	luvi_increase: Option<Decimal>,

	#[serde(rename = "priceShiftLoss", with = "rust_decimal::serde::str")]
	price_shift_loss: Decimal,

	#[serde(rename = "startAssetDepth", deserialize_with = "deserialize_number_from_string")]
	start_asset_depth: u64,

	#[serde(rename = "startLPUnits", deserialize_with = "deserialize_number_from_string")]
	start_lp_units: u64,

	#[serde(rename = "startMemberCount", deserialize_with = "deserialize_number_from_string")]
	start_member_count: u64,

	#[serde(rename = "startRuneDepth", deserialize_with = "deserialize_number_from_string")]
	start_rune_depth: u64,

	/// Signed: these go negative on pools whose synths have been burnt
	/// below the recorded baseline.
	#[serde(rename = "startSynthUnits", deserialize_with = "deserialize_number_from_string")]
	start_synth_units: i64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,
}

impl DepthHistoryMeta {
	#[must_use]
	pub const fn get_end_asset_depth(&self) -> &u64 {
		&self.end_asset_depth
	}

	#[must_use]
	pub const fn get_end_lp_units(&self) -> &u64 {
		&self.end_lp_units
	}

	#[must_use]
	pub const fn get_end_member_count(&self) -> &u64 {
		&self.end_member_count
	}

	#[must_use]
	pub const fn get_end_rune_depth(&self) -> &u64 {
		&self.end_rune_depth
	}

	#[must_use]
	pub const fn get_end_synth_units(&self) -> &i64 {
		&self.end_synth_units
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_luvi_increase(&self) -> &Option<Decimal> {
		&self.luvi_increase
	}

	#[must_use]
	pub const fn get_price_shift_loss(&self) -> &Decimal {
		&self.price_shift_loss
	}

	#[must_use]
	pub const fn get_start_asset_depth(&self) -> &u64 {
		&self.start_asset_depth
	}

	#[must_use]
	pub const fn get_start_lp_units(&self) -> &u64 {
		&self.start_lp_units
	}

	#[must_use]
	pub const fn get_start_member_count(&self) -> &u64 {
		&self.start_member_count
	}

	#[must_use]
	pub const fn get_start_rune_depth(&self) -> &u64 {
		&self.start_rune_depth
	}

	#[must_use]
	pub const fn get_start_synth_units(&self) -> &i64 {
		&self.start_synth_units
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}
}

#[cfg(test)]
mod tests {
	use chrono::DateTime;

	use super::*;

	// Deserialize the DepthHistoryMeta from a JSON string and compare it to the expected DepthHistoryMeta
	#[test]
	fn test_deserialize_depth_history_meta() {
		let json = r#"{
                        "endAssetDepth": "20789919007903",
                        "endLPUnits": "901529373373",
                        "endMemberCount": "202",
                        "endRuneDepth": "3324827631133",
                        "endSynthUnits": "59635896754",
                        "endTime": "1710892800",
                        "luviIncrease": "1.0137411270039827",
                        "priceShiftLoss": "0.9964447262207828",
                        "startAssetDepth": "20365684205644",
                        "startLPUnits": "972814765176",
                        "startMemberCount": "211",
                        "startRuneDepth": "3856266522266",
                        "startSynthUnits": "65782544741",
                        "startTime": "1710028800"
                }"#;
		let expected = DepthHistoryMeta {
			end_asset_depth: 20789919007903,
			end_lp_units: 901529373373,
			end_member_count: 202,
			end_rune_depth: 3324827631133,
			end_synth_units: 59635896754,
			end_time: DateTime::from_timestamp(1710892800, 0).expect("failed to create DateTime"),
			luvi_increase: Some(Decimal::new(10137411270039827, 16)),
			price_shift_loss: Decimal::new(9964447262207828, 16),
			start_asset_depth: 20365684205644,
			start_lp_units: 972814765176,
			start_member_count: 211,
			start_rune_depth: 3856266522266,
			start_synth_units: 65782544741,
			start_time: DateTime::from_timestamp(1710028800, 0).expect("failed to create DateTime"),
		};
		let deserialized: DepthHistoryMeta = serde_json::from_str(json).unwrap();
		assert_eq!(deserialized, expected);
	}

	/// The real `meta` block `history/depths/BTC.BTC?interval=day&count=100`
	/// returned on 2026-09-24. It carries `"NaN"` for `luviIncrease` and
	/// negative start/end synth unit counts; before those two fields were
	/// widened this payload failed to deserialize at all.
	#[test]
	fn test_deserialize_depth_history_meta_with_nan_and_negative_units() {
		let json = r#"{
			"endAssetDepth": "119384894669",
			"endLPUnits": "232779266143857",
			"endMemberCount": "2998",
			"endRuneDepth": "1546268565784675",
			"endSynthUnits": "-659220551852742",
			"endTime": "1790294400",
			"luviIncrease": "NaN",
			"priceShiftLoss": "0.9999719781546587",
			"startAssetDepth": "119412894669",
			"startLPUnits": "233009705453414",
			"startMemberCount": "2998",
			"startRuneDepth": "1546984565784675",
			"startSynthUnits": "-649381483597292",
			"startTime": "1781654400"
		}"#;
		let deserialized: DepthHistoryMeta = serde_json::from_str(json).unwrap();
		assert_eq!(deserialized.get_luvi_increase(), &None);
		assert_eq!(deserialized.get_start_synth_units(), &-649_381_483_597_292_i64);
		assert_eq!(deserialized.get_end_synth_units(), &-659_220_551_852_742_i64);
	}
}

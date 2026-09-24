//! Deserialization helpers for Midgard's decimal-valued string fields.
//!
//! Midgard encodes every numeric field as a JSON string, and for ratio-shaped
//! fields it emits the literal string `"NaN"` whenever the ratio is undefined —
//! a LUVI over a pool with no liquidity, for example. `rust_decimal` has no NaN
//! representation, so `rust_decimal::serde::str` rejects those values outright
//! and the whole response fails to parse.
//!
//! [`optional`] models that shape honestly: `Some(value)` for a real number and
//! `None` for `"NaN"`, so a consumer can tell "undefined" from a genuine zero.

/// A `Decimal` field that upstream may report as the string `"NaN"`.
///
/// `"NaN"` deserializes to `None`; anything else goes through
/// `rust_decimal::serde::str`. Serializing `None` writes `"NaN"` back, so a
/// round-trip through this crate preserves what Midgard actually said.
pub mod optional {
	use rust_decimal::Decimal;
	use serde::de::Error as _;
	use serde::{Deserialize, Deserializer, Serializer};

	/// # Errors
	/// Returns an error if the field is not a string, or is a string that is
	/// neither `"NaN"` nor a value `rust_decimal` can parse.
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;
		if super::is_nan(&raw) {
			return Ok(None);
		}
		raw.parse::<Decimal>().map(Some).map_err(D::Error::custom)
	}

	/// # Errors
	/// Returns an error if the underlying serializer does.
	//
	// clippy::ref_option would prefer `Option<&Decimal>`, but the signature of a
	// `#[serde(with = ...)]` module's `serialize` is fixed by serde: it is always
	// called with a reference to the field. Narrowing the allow to this one
	// function keeps the lint live everywhere else.
	#[allow(clippy::ref_option)]
	pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match value {
			Some(value) => serializer.serialize_str(&value.to_string()),
			None => serializer.serialize_str("NaN"),
		}
	}
}

/// Whether a raw Midgard field holds the "undefined" sentinel rather than a number.
///
/// Matched case-insensitively, and `"nan"`, `"+nan"` and `"-nan"` are all treated
/// the same, because which of them upstream emits is not something this crate
/// should depend on.
fn is_nan(raw: &str) -> bool {
	let trimmed = raw.trim();
	let unsigned = trimmed.strip_prefix(['+', '-']).unwrap_or(trimmed);
	unsigned.eq_ignore_ascii_case("nan")
}

/// Parses a decimal the way [`optional`] does, for callers that already hold the
/// raw string. Kept alongside [`is_nan`] so the two definitions cannot drift.
#[cfg(test)]
fn parse(raw: &str) -> Option<rust_decimal::Decimal> {
	if is_nan(raw) {
		None
	} else {
		raw.trim().parse::<rust_decimal::Decimal>().ok()
	}
}

#[cfg(test)]
mod tests {
	use rust_decimal::Decimal;
	use serde::{Deserialize, Serialize};

	use super::{is_nan, parse};

	#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
	struct Holder {
		#[serde(with = "super::optional")]
		luvi: Option<Decimal>,
	}

	#[test]
	fn recognises_the_nan_sentinel() {
		for raw in ["NaN", "nan", "NAN", "-NaN", "+nan", " NaN "] {
			assert!(is_nan(raw), "{raw:?} should be recognised as NaN");
		}
	}

	#[test]
	fn does_not_mistake_numbers_for_nan() {
		for raw in ["0", "-1.5", "1e3", "", "NaNa", "not-a-number"] {
			assert!(!is_nan(raw), "{raw:?} should not be recognised as NaN");
		}
	}

	#[test]
	fn parses_real_values() {
		assert_eq!(parse("1.25"), Some(Decimal::new(125, 2)));
		assert_eq!(parse("-416371778143878"), Some(Decimal::from(-416_371_778_143_878_i64)));
		assert_eq!(parse("NaN"), None);
	}

	/// The exact `luvi` value the live `history/depths/BTC.BTC` endpoint
	/// returned on 2026-09-24, which used to fail the whole response.
	#[test]
	fn deserializes_the_nan_luvi_that_broke_depth_history() {
		let holder: Holder = serde_json::from_str(r#"{"luvi":"NaN"}"#).unwrap();
		assert_eq!(holder.luvi, None);
	}

	#[test]
	fn deserializes_a_real_luvi() {
		let holder: Holder = serde_json::from_str(r#"{"luvi":"1.0000123"}"#).unwrap();
		assert_eq!(holder.luvi, Some("1.0000123".parse::<Decimal>().unwrap()));
	}

	#[test]
	fn round_trips_through_serialization() {
		for json in [r#"{"luvi":"NaN"}"#, r#"{"luvi":"1.0000123"}"#] {
			let holder: Holder = serde_json::from_str(json).unwrap();
			let back = serde_json::to_string(&holder).unwrap();
			assert_eq!(back, json, "round trip changed {json}");
			let again: Holder = serde_json::from_str(&back).unwrap();
			assert_eq!(again, holder);
		}
	}

	#[test]
	fn rejects_a_value_that_is_neither_a_number_nor_nan() {
		let err = serde_json::from_str::<Holder>(r#"{"luvi":"banana"}"#).unwrap_err();
		assert!(err.to_string().contains("Invalid decimal") || err.to_string().contains("banana"), "unexpected error: {err}");
	}
}

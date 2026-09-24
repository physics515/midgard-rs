//! Deserializer for a Midgard field that is a stringified number *and* may be
//! absent or null.
//!
//! `serde_aux::prelude::deserialize_option_number_from_string` almost fits, but
//! its inner untagged enum has a borrowed `&str` variant, so it fails with
//! "data did not match any variant of untagged enum NumericOrNull" whenever the
//! input cannot lend out a string — most notably `serde_json::from_value`,
//! which a consumer holding a `Value` would reasonably call. This helper takes
//! the string by value instead, so it works for both `from_str` and
//! `from_value`.

use std::fmt;
use std::str::FromStr;

use serde::de::{Deserializer, Error, Unexpected, Visitor};

/// Deserializes `Option<T>` from a JSON string, a JSON number, null, or an
/// absent field (with `#[serde(default)]`).
///
/// # Errors
/// Returns an error if the value is present but is neither a number nor a
/// string that parses as one.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr + TryFrom<u64> + TryFrom<i64>,
{
	struct OptionalNumber<T>(std::marker::PhantomData<T>);

	impl<'de, T> Visitor<'de> for OptionalNumber<T>
	where
		T: FromStr + TryFrom<u64> + TryFrom<i64>,
	{
		type Value = Option<T>;

		fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.write_str("a number, a string holding a number, or null")
		}

		fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
			Ok(None)
		}

		fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
			Ok(None)
		}

		fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_any(Self(std::marker::PhantomData))
		}

		fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
			let trimmed = value.trim();
			if trimmed.is_empty() {
				return Ok(None);
			}
			trimmed.parse::<T>().map(Some).map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
		}

		fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
			self.visit_str(&value)
		}

		fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
			T::try_from(value).map(Some).map_err(|_| E::invalid_value(Unexpected::Unsigned(value), &self))
		}

		fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
			T::try_from(value).map(Some).map_err(|_| E::invalid_value(Unexpected::Signed(value), &self))
		}
	}

	deserializer.deserialize_option(OptionalNumber(std::marker::PhantomData))
}

#[cfg(test)]
mod tests {
	use serde::Deserialize;
	use serde_json::json;

	#[derive(Deserialize, Debug, PartialEq, Eq)]
	struct Holder {
		#[serde(default, deserialize_with = "super::deserialize")]
		height: Option<u64>,
	}

	#[test]
	fn reads_a_stringified_number() {
		let holder: Holder = serde_json::from_str(r#"{"height":"15125786"}"#).unwrap();
		assert_eq!(holder.height, Some(15_125_786));
	}

	#[test]
	fn reads_a_bare_number() {
		let holder: Holder = serde_json::from_str(r#"{"height":15125786}"#).unwrap();
		assert_eq!(holder.height, Some(15_125_786));
	}

	#[test]
	fn an_absent_field_is_none() {
		let holder: Holder = serde_json::from_str("{}").unwrap();
		assert_eq!(holder.height, None);
	}

	#[test]
	fn null_and_empty_string_are_none() {
		assert_eq!(serde_json::from_str::<Holder>(r#"{"height":null}"#).unwrap().height, None);
		assert_eq!(serde_json::from_str::<Holder>(r#"{"height":""}"#).unwrap().height, None);
	}

	/// The reason this helper exists rather than `serde_aux`'s: that one has a
	/// borrowed `&str` variant and fails outright here.
	#[test]
	fn works_when_deserializing_from_a_value() {
		let holder: Holder = serde_json::from_value(json!({"height": "15125786"})).unwrap();
		assert_eq!(holder.height, Some(15_125_786));

		let holder: Holder = serde_json::from_value(json!({})).unwrap();
		assert_eq!(holder.height, None);
	}

	#[test]
	fn rejects_a_value_that_is_not_a_number() {
		assert!(serde_json::from_str::<Holder>(r#"{"height":"banana"}"#).is_err());
	}
}

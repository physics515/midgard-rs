//! The one place this crate talks to a Midgard instance over HTTP.
//!
//! Every `api_*` function funnels through [`get_body`], so the request, the
//! status check and the body read are written once rather than twenty-five
//! times, and every request identifies this crate to the instance it calls.

use anyhow::{bail, Result};
use reqwest::Client;

use crate::APIError;

/// Identifies this crate to the Midgard instance being called.
///
/// Public gateways ask callers to identify themselves — Liquify uses it to
/// raise the per-IP rate limit for known applications — and an instance
/// operator who needs to trace or throttle traffic should be able to tell what
/// generated it.
const CLIENT_ID: &str = concat!("midgard-rs/", env!("CARGO_PKG_VERSION"));

/// Builds the client for a single request.
///
/// A per-request client is wasteful — it rebuilds the connection pool and TLS
/// configuration every time — and the obvious fix is one shared client. That
/// fix was tried here and reverted, because a `static` `reqwest::Client`
/// cannot be shared across tokio runtimes: its pooled connections are driven
/// by background tasks belonging to whichever runtime first used them, so once
/// that runtime is dropped every later request through the same client fails
/// with "runtime dropped the dispatch task" or "connection closed". That is
/// not only a test artefact — `#[tokio::test]` builds a runtime per test, and
/// so does any consumer that uses more than one.
///
/// The client therefore has to live on [`crate::Midgard`], which is where a
/// consumer's runtime lifetime is. That is a deliberate API change (`Midgard`
/// derives `Serialize`/`Deserialize`, so the field needs `#[serde(skip)]`) and
/// is filed in `ROADMAP.md` rather than rushed.
///
/// # Errors
/// Returns [`APIError::ClientBuild`] if `reqwest` cannot initialise its TLS
/// backend.
fn client() -> Result<Client> {
	match Client::builder().user_agent(CLIENT_ID).build() {
		Ok(client) => Ok(client),
		Err(e) => bail!(APIError::ClientBuild(e.to_string())),
	}
}

/// How much of an error response body to keep in the error message.
///
/// Enough to recognise a rate-limit notice or an HTML error page, short enough
/// that a 500 KB response does not end up in a log line.
const MAX_ERROR_BODY: usize = 512;

/// Performs a GET against a Midgard endpoint and returns the response body.
///
/// The status is checked before the body is handed back, which is the part
/// that used to be missing: without it a 404, a 429 or an HTML error page from
/// a proxy went straight into `serde_json` and surfaced as
/// `Serde Error: expected value at line 1 column 1`, telling the caller
/// nothing about what actually went wrong.
///
/// # Errors
/// * [`APIError::ClientBuild`] if the shared HTTP client could not be built.
/// * [`APIError::ReqwestError`] if the request could not be made, or the body
///   could not be read.
/// * [`APIError::HttpStatus`] if the instance answered with a non-success
///   status.
pub async fn get_body(endpoint: &str) -> Result<String> {
	let response = match client()?.get(endpoint).header("x-client-id", CLIENT_ID).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let status = response.status();

	let body = match response.text().await {
		Ok(body) => body,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	if !status.is_success() {
		bail!(APIError::HttpStatus { status: status.as_u16(), url: endpoint.to_string(), body: truncate(&body) });
	}

	Ok(body)
}

/// Shortens `body` to [`MAX_ERROR_BODY`] characters, on a character boundary so
/// that a multi-byte character is never split.
fn truncate(body: &str) -> String {
	let trimmed = body.trim();
	if trimmed.chars().count() <= MAX_ERROR_BODY {
		return trimmed.to_string();
	}

	let kept: String = trimmed.chars().take(MAX_ERROR_BODY).collect();
	format!("{kept}… (truncated)")
}

#[cfg(test)]
mod tests {
	use super::{truncate, MAX_ERROR_BODY};

	/// Building a client performs no I/O, so this belongs with the offline
	/// tests. It guards the TLS backend initialising at all, which is the one
	/// way `client()` can fail.
	#[test]
	fn a_client_can_be_built() {
		assert!(super::client().is_ok());
	}

	#[test]
	fn short_bodies_are_kept_whole() {
		assert_eq!(truncate("not found"), "not found");
		assert_eq!(truncate("  spaced  "), "spaced");
		assert_eq!(truncate(""), "");
	}

	#[test]
	fn long_bodies_are_truncated() {
		let body = "x".repeat(MAX_ERROR_BODY + 100);
		let truncated = truncate(&body);
		assert!(truncated.ends_with("… (truncated)"));
		assert_eq!(truncated.chars().count(), MAX_ERROR_BODY + "… (truncated)".chars().count());
	}

	/// Truncating by bytes would panic here; truncating by characters must not.
	#[test]
	fn multi_byte_characters_are_not_split() {
		let body = "é".repeat(MAX_ERROR_BODY + 10);
		let truncated = truncate(&body);
		assert!(truncated.starts_with('é'));
		assert!(truncated.ends_with("… (truncated)"));
	}

	#[test]
	fn a_body_exactly_at_the_limit_is_not_truncated() {
		let body = "y".repeat(MAX_ERROR_BODY);
		assert_eq!(truncate(&body), body);
	}
}

#[cfg(test)]
mod live_tests {
	use super::get_body;

	/// Before the status check existed, a 404 from a Midgard instance was fed
	/// to `serde_json` and reported as `Serde Error: expected value at line 1
	/// column 1`. It must now say what actually happened.
	#[tokio::test]
	async fn a_404_is_reported_as_an_http_error_not_a_parse_error() {
		let endpoint = format!("{}thorname/lookup/{}", crate::DEFAULT_BASE_URL, "a-thorname-that-does-not-exist-90210");
		let err = match get_body(&endpoint).await {
			Ok(body) => panic!("expected an error, got a body: {body}"),
			Err(e) => e,
		};

		let message = err.to_string();
		assert!(message.starts_with("HTTP 404"), "expected an HTTP status error, got: {message}");
		assert!(message.contains(&endpoint), "the error should name the URL it failed on, got: {message}");
		assert!(!message.contains("Serde Error"), "a 404 must not be reported as a parse failure, got: {message}");
	}

	/// Public gateways ask callers to identify themselves, so check the headers
	/// actually arrive rather than trusting that they were configured.
	#[tokio::test]
	async fn requests_identify_this_crate() {
		let body = get_body("https://httpbin.org/headers").await.unwrap();
		let expected = format!("midgard-rs/{}", env!("CARGO_PKG_VERSION"));
		assert!(body.contains(&expected), "expected {expected} in the echoed headers, got: {body}");
		assert!(body.to_lowercase().contains("x-client-id"), "x-client-id was not sent, got: {body}");
	}

	#[tokio::test]
	async fn a_successful_request_returns_the_body() {
		let endpoint = format!("{}health", crate::DEFAULT_BASE_URL);
		let body = get_body(&endpoint).await.unwrap();
		assert!(body.contains("lastThorNode"), "unexpected health body: {body}");
	}
}

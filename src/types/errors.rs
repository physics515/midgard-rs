use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum APIError {
	#[error("Reqwest Error: {0}")]
	ReqwestError(#[from] reqwest::Error),
	#[error("Serde Error: {0}")]
	SerdeError(#[from] serde_json::Error),
	#[error("Invalid Parameter: {0}")]
	InvalidParameter(String),

	/// The shared HTTP client could not be constructed.
	///
	/// Effectively unreachable in normal use — it means `reqwest` could not
	/// initialise its TLS backend — but a library should not panic on it.
	#[error("Failed to build the HTTP client: {0}")]
	ClientBuild(String),

	/// The Midgard instance answered, but with a non-success HTTP status.
	///
	/// Before this variant existed the body of such a response was handed
	/// straight to `serde_json`, so a 404 or a proxy's HTML error page
	/// surfaced as a confusing [`APIError::SerdeError`].
	#[error("HTTP {status} from {url}: {body}")]
	HttpStatus {
		/// The status code the instance returned.
		status: u16,
		/// The URL that was requested.
		url: String,
		/// The response body, truncated to keep the message readable.
		body: String,
	},
}

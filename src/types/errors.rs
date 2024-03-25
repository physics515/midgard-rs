use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
	#[error("Reqwest Error: {0}")]
	ReqwestError(#[from] reqwest::Error),
	#[error("Serde Error: {0}")]
	SerdeError(#[from] serde_json::Error),
	#[error("Invalid Parameter: {0}")]
	InvalidParameter(String),
}

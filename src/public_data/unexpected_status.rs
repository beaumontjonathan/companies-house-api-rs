use derive_more::derive::Display;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error, Display)]
#[display("Unexpected {status_code}")]
pub struct UnexpectedStatusError {
    pub status_code: StatusCode,
}

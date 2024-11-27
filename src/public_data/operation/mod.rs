pub mod get_company_profile;
pub mod get_company_registered_office_address;
pub mod search_companies;
pub mod search_disqualified_officers;
pub mod search_officers;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompaniesHousePublicDataOperationError<StatusError: std::error::Error> {
    #[error("Reqwest error: {inner}")]
    Reqwest {
        #[from]
        inner: reqwest::Error,
    },

    #[error("Unable to parse json body: {inner}")]
    JsonParse {
        inner: serde_json::Error,
        /// If the body can be parsed as JSON, but simply does not match the
        /// operation data type, this value will be present.
        value: Option<serde_json::Value>,
        body: bytes::Bytes,
    },

    #[error(transparent)]
    Status(StatusError),
}

pub trait CompaniesHousePublicDataOperation {
    type StatusError: std::error::Error;
    type Data: Debug + DeserializeOwned;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError<Self::StatusError>>;

    fn handle_status(&self, response: StatusCode) -> Result<(), Self::StatusError>;
}

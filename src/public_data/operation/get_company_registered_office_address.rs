use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};
use crate::{public_data::types, unrecognised_response::UnrecognisedResponse};

/// Get the current address of a company
#[derive(TypedBuilder)]
pub struct GetCompanyRegisteredOfficeAddress {
    #[builder(setter(into))]
    company_number: String,
}

#[derive(Debug, Error)]
pub enum GetCompanyRegisteredOfficeAddressError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Operation error")]
    OperationError(#[from] CompaniesHousePublicDataOperationError),
    #[error("Bad JSON")]
    SerdeJson(#[from] reqwest::Error),
    #[error("UnrecognisedResponse {0:?}")]
    UnrecognisedResponse(#[from] UnrecognisedResponse),
}

impl CompaniesHousePublicDataOperation for GetCompanyRegisteredOfficeAddress {
    type Error = GetCompanyRegisteredOfficeAddressError;
    type Data = types::RegisteredOfficeAddress;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError> {
        Ok(client
            .get(format!(
                "{base_url}/company/{}/registered-office-address",
                self.company_number
            ))
            .build()?)
    }

    async fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> Result<Self::Data, Self::Error> {
        match response.status() {
            StatusCode::UNAUTHORIZED => return Err(Self::Error::Unauthorized),
            StatusCode::NOT_FOUND => return Err(Self::Error::NotFound),
            StatusCode::OK => {}
            _ => {
                return Err(Self::Error::UnrecognisedResponse(
                    UnrecognisedResponse::from_response(response).await,
                ))
            }
        };

        let value: Self::Data = response
            .json()
            .await
            .map_err(CompaniesHousePublicDataOperationError::Reqwest)?;

        Ok(value)
    }
}

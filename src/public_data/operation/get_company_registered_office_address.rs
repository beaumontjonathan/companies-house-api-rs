use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};
use crate::{public_data::types, unexpected_status::UnexpectedStatusError};

/// Get the current address of a company
#[derive(TypedBuilder)]
pub struct GetCompanyRegisteredOfficeAddress {
    #[builder(setter(into))]
    company_number: String,
}

#[derive(Debug, Error)]
pub enum GetCompanyRegisteredOfficeAddressStatusError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    UnexpectedStatus(#[from] UnexpectedStatusError),
}

impl CompaniesHousePublicDataOperation for GetCompanyRegisteredOfficeAddress {
    type StatusError = GetCompanyRegisteredOfficeAddressStatusError;
    type Data = types::RegisteredOfficeAddress;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError<Self::StatusError>> {
        Ok(client
            .get(format!(
                "{base_url}/company/{}/registered-office-address",
                self.company_number
            ))
            .build()?)
    }

    fn handle_status(&self, status_code: StatusCode) -> Result<(), Self::StatusError> {
        match status_code {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(Self::StatusError::Unauthorized),
            StatusCode::NOT_FOUND => Err(Self::StatusError::NotFound),
            status_code => Err(Self::StatusError::UnexpectedStatus(UnexpectedStatusError {
                status_code,
            })),
        }
    }
}

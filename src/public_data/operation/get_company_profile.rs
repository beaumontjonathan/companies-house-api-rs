use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};
use crate::{types::CompanyProfile, unexpected_status::UnexpectedStatusError};

/// Get the basic company information
#[derive(TypedBuilder)]
pub struct GetCompanyProfile {
    #[builder(setter(into))]
    company_number: String,
}

#[derive(Debug, Error)]
pub enum GetCompanyProfileStatusError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    UnexpectedStatus(#[from] UnexpectedStatusError),
}

impl CompaniesHousePublicDataOperation for GetCompanyProfile {
    type StatusError = GetCompanyProfileStatusError;
    type Data = CompanyProfile;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError<Self::StatusError>> {
        Ok(client
            .get(format!("{base_url}/company/{}", self.company_number))
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

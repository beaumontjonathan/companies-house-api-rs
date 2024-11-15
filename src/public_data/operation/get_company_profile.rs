use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::{types, unrecognised_response::UnrecognisedResponse};

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};

/// Get the basic company information
#[derive(TypedBuilder)]
pub struct GetCompanyProfile {
    #[builder(setter(into))]
    company_number: String,
}

#[derive(Debug, Error)]
pub enum GetCompanyProfileError {
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

impl CompaniesHousePublicDataOperation for GetCompanyProfile {
    type Error = GetCompanyProfileError;
    type Data = types::CompanyProfile;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError> {
        Ok(client
            .get(format!("{base_url}/company/{}", self.company_number))
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

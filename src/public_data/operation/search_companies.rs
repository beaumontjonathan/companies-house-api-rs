use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::{types::CompanySearch, unrecognised_response::UnrecognisedResponse};

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};

/// Search company information
#[derive(TypedBuilder)]
pub struct SearchCompanies {
    /// The term being searched for.
    #[builder(setter(into))]
    query: String,
    /// The number of search results to return per page.
    #[builder(default)]
    items_per_page: Option<usize>,
    /// The index of the first result item to return.
    #[builder(default)]
    start_index: Option<usize>,
    /// Enumerable options to restrict search results. Space separate multiple restriction options
    /// to combine functionality. For a "company name availability" search use
    /// "active-companies legally-equivalent-company-name" together.
    #[builder(default)]
    restrictions: Option<String>,
}

#[derive(Debug, Error)]
pub enum SearchCompaniesError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Operation error")]
    OperationError(#[from] CompaniesHousePublicDataOperationError),
    #[error("Bad JSON")]
    SerdeJson(#[from] reqwest::Error),
    #[error("UnrecognisedResponse {0:?}")]
    UnrecognisedResponse(#[from] UnrecognisedResponse),
}

impl CompaniesHousePublicDataOperation for SearchCompanies {
    type Error = SearchCompaniesError;
    type Data = CompanySearch;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError> {
        let mut query_params = vec![("q", self.query.to_owned())];

        if let Some(items_per_page) = self.items_per_page {
            query_params.push(("items_per_page", items_per_page.to_string()))
        }

        if let Some(start_index) = self.start_index {
            query_params.push(("start_index", start_index.to_string()));
        }

        if let Some(restrictions) = &self.restrictions {
            query_params.push(("restrictions", restrictions.to_owned()));
        }

        Ok(client
            .get(format!("{base_url}/search/companies"))
            .query(&query_params)
            .build()?)
    }

    async fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> Result<Self::Data, Self::Error> {
        match response.status() {
            StatusCode::UNAUTHORIZED => return Err(Self::Error::Unauthorized),
            StatusCode::OK => {}
            _ => {
                return Err(Self::Error::UnrecognisedResponse(
                    UnrecognisedResponse::from_response(response).await,
                ))
            }
        }

        let value: Self::Data = response
            .json()
            .await
            .map_err(CompaniesHousePublicDataOperationError::Reqwest)?;

        Ok(value)
    }
}

use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::{types::DisqualifiedOfficerSearch, unrecognised_response::UnrecognisedResponse};

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};

/// Search for disqualified officer information
/// https://developer-specs.company-information.service.gov.uk/companies-house-public-data-api/reference/search/search-disqualified-officers
#[derive(TypedBuilder)]
pub struct SearchDisqualifiedOfficers {
    /// The term being searched for.
    #[builder(setter(into))]
    query: String,
    /// The number of search results to return per page.
    #[builder(default)]
    items_per_page: Option<usize>,
    /// The index of the first result item to return.
    #[builder(default)]
    start_index: Option<usize>,
}

#[derive(Debug, Error)]
pub enum SearchDisqualifiedOfficersError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Operation error")]
    OperationError(#[from] CompaniesHousePublicDataOperationError),
    #[error("Bad JSON")]
    SerdeJson(#[from] reqwest::Error),
    #[error("UnrecognisedResponse {0:?}")]
    UnrecognisedResponse(#[from] UnrecognisedResponse),
}

impl CompaniesHousePublicDataOperation for SearchDisqualifiedOfficers {
    type Error = SearchDisqualifiedOfficersError;
    type Data = DisqualifiedOfficerSearch;

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

        Ok(client
            .get(format!("{base_url}/search/disqualified-officers"))
            .query(&query_params)
            .build()?)
    }

    async fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> Result<Self::Data, Self::Error> {
        match response.status() {
            StatusCode::UNAUTHORIZED => {
                return Err(Self::Error::Unauthorized);
            }
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

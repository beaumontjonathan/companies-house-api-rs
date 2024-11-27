use reqwest::StatusCode;
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::{types::OfficerSearch, unexpected_status::UnexpectedStatusError};

use super::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};

/// Search company information
#[derive(TypedBuilder)]
pub struct SearchOfficers {
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
pub enum SearchOfficersStatusError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedStatus(#[from] UnexpectedStatusError),
}

impl CompaniesHousePublicDataOperation for SearchOfficers {
    type StatusError = SearchOfficersStatusError;
    type Data = OfficerSearch;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError<Self::StatusError>> {
        let mut query_params = vec![("q", self.query.to_owned())];

        if let Some(items_per_page) = self.items_per_page {
            query_params.push(("items_per_page", items_per_page.to_string()))
        }

        if let Some(start_index) = self.start_index {
            query_params.push(("start_index", start_index.to_string()));
        }

        Ok(client
            .get(format!("{base_url}/search/officers"))
            .query(&query_params)
            .build()?)
    }

    fn handle_status(&self, status_code: StatusCode) -> Result<(), Self::StatusError> {
        match status_code {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(Self::StatusError::Unauthorized),
            status_code => Err(Self::StatusError::UnexpectedStatus(UnexpectedStatusError {
                status_code,
            })),
        }
    }
}

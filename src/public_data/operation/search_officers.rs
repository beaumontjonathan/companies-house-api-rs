use reqwest::StatusCode;
use serde_json::Value;
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::types::OfficerSearch;

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
pub enum SearchOfficersError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Operation error")]
    OperationError(#[from] CompaniesHousePublicDataOperationError),
    #[error("Bad JSON")]
    SerdeJson(#[from] reqwest::Error),
}

impl CompaniesHousePublicDataOperation for SearchOfficers {
    type Error = SearchOfficersError;
    type Data = OfficerSearch;

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
            .get(format!("{base_url}/search/officers"))
            .query(&query_params)
            .build()?)
    }

    async fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> Result<Self::Data, Self::Error> {
        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(Self::Error::Unauthorized);
        };

        let value: Value = response
            .json()
            .await
            .map_err(CompaniesHousePublicDataOperationError::Reqwest)?;

        dbg!(&value);

        let value: Self::Data = serde_json::from_value(value).unwrap();

        Ok(value)
    }
}
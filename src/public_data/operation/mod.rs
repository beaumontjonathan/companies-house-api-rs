pub mod get_company_profile;
pub mod get_company_registered_office_address;
pub mod search_companies;
pub mod search_officers;

use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompaniesHousePublicDataOperationError {
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
}

pub trait CompaniesHousePublicDataOperation {
    type Error: std::error::Error + From<CompaniesHousePublicDataOperationError>;
    type Data: Debug;

    fn build_request(
        &self,
        base_url: &str,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request, CompaniesHousePublicDataOperationError>;

    fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> impl std::future::Future<Output = Result<Self::Data, Self::Error>>;
}

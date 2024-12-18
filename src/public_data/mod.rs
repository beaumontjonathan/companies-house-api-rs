use operation::{CompaniesHousePublicDataOperation, CompaniesHousePublicDataOperationError};
use thiserror::Error;

pub mod operation;
pub mod types;
pub mod unexpected_status;

#[derive(Debug, Error)]
pub enum CompaniesHousePublicDataClientError {
    #[error("Bad Companies House API key provided")]
    BadApiKey,
    #[error("Unable to construct client")]
    ReqwestError(#[from] reqwest::Error),
}

pub struct CompaniesHousePublicDataClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl CompaniesHousePublicDataClient {
    fn build_client(api_key: &str) -> Result<reqwest::Client, CompaniesHousePublicDataClientError> {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        let mut header_value = header::HeaderValue::from_str(api_key)
            .map_err(|_| CompaniesHousePublicDataClientError::BadApiKey)?;
        header_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, header_value);

        Ok(reqwest::Client::builder()
            .default_headers(headers)
            .build()?)
    }

    pub fn new(api_key: &str) -> Result<Self, CompaniesHousePublicDataClientError> {
        let client = Self::build_client(api_key)?;
        Ok(CompaniesHousePublicDataClient {
            api_key: api_key.to_string(),
            client,
            base_url: "https://api.company-information.service.gov.uk".to_string(),
        })
    }

    pub fn rotate_client(&mut self) -> Result<(), CompaniesHousePublicDataClientError> {
        self.client = Self::build_client(&self.api_key)?;
        Ok(())
    }

    pub async fn send<T: CompaniesHousePublicDataOperation>(
        &self,
        operation: T,
    ) -> Result<T::Data, CompaniesHousePublicDataOperationError<T::StatusError>> {
        let request = operation.build_request(&self.base_url, &self.client)?;

        let response = self.client.execute(request).await?;

        operation
            .handle_status(response.status())
            .map_err(CompaniesHousePublicDataOperationError::Status)?;

        let bytes = response.bytes().await?;

        match serde_json::from_slice(&bytes) {
            Ok(value) => Ok(value),
            Err(inner) => Err(CompaniesHousePublicDataOperationError::JsonParse {
                inner,
                value: serde_json::from_slice(&bytes).ok(),
                body: bytes,
            }),
        }
    }
}

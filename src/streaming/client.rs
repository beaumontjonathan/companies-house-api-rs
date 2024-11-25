use super::{
    operation::CompaniesHouseStreamingOperation, CompaniesHouseStreamingConnectionError,
    StreamConnection,
};
use bytes::BytesMut;
use reqwest::StatusCode;
use std::time::Duration;
use tokio::time::timeout;

const STREAMING_BASE_URL: &str = "https://stream.companieshouse.gov.uk";

pub struct CompaniesHouseStreamingClient {
    api_key: String,
    connection_timeout: Duration,
    chunk_timeout: Duration,
}

impl CompaniesHouseStreamingClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            connection_timeout: Duration::from_secs(60),
            chunk_timeout: Duration::from_secs(60),
        }
    }

    pub fn connection_timeout(self, duration: Duration) -> Self {
        Self {
            connection_timeout: duration,
            ..self
        }
    }

    pub fn chunk_timeout(self, duration: Duration) -> Self {
        Self {
            chunk_timeout: duration,
            ..self
        }
    }

    pub async fn stream<O: CompaniesHouseStreamingOperation>(
        &self,
        operation: O,
        timepoint: Option<usize>,
    ) -> Result<StreamConnection<O>, CompaniesHouseStreamingConnectionError> {
        match &timepoint {
            Some(t) => log::info!("Connecting to stream with timepoint {t}"),
            None => log::info!("Connecting to stream without timepoint..."),
        };

        let mut request = reqwest::Client::new()
            .get(format!("{}{}", STREAMING_BASE_URL, operation.endpoint()))
            .basic_auth(&self.api_key, Option::<&str>::None);

        if let Some(timepoint) = timepoint {
            request = request.query(&[("timepoint", timepoint)]);
        }

        let response = timeout(self.connection_timeout, request.send())
            .await
            .map_err(|_| {
                log::info!("Connection timeout");
                CompaniesHouseStreamingConnectionError::ConnectionTimeout
            })?
            .map_err(CompaniesHouseStreamingConnectionError::UnknownConnection)?;

        log::info!(status = response.status().as_u16(); "Connection successful");

        match response.status() {
            StatusCode::OK => Ok(StreamConnection {
                buffer: BytesMut::new(),
                max_chunk_timeout: self.chunk_timeout,
                response,
                _operation: operation,
            }),
            StatusCode::RANGE_NOT_SATISFIABLE => {
                Err(CompaniesHouseStreamingConnectionError::BadTimepoint)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err(CompaniesHouseStreamingConnectionError::TooManyRequests)
            }
            StatusCode::UNAUTHORIZED => Err(CompaniesHouseStreamingConnectionError::Unauthorized),
            status_code => Err(CompaniesHouseStreamingConnectionError::UnknownResponse(
                status_code,
            )),
        }
    }
}

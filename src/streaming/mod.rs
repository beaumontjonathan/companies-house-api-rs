use bytes::BytesMut;
use std::{str::from_utf8, time::Duration};
use thiserror::Error;
use tokio::time::timeout;

#[derive(Debug, Error)]
pub enum CompaniesHouseStreamingError {
    #[error("Connection timeout exceeded")]
    ConnectionTimeout,
    #[error("Unknown error connecting to stream endpoint")]
    UnknownConnection(reqwest::Error),
    #[error("Unable to read utf8 from response body")]
    BadValueEncoding(std::str::Utf8Error),
    #[error("Unable to deserialize next stream item as JSON")]
    BadValueJson(serde_json::Error),
    #[error("Stream chunk timeout exceeded")]
    ChunkTimeout,
    #[error("Unable to read from response body")]
    BadChunk(reqwest::Error),
    #[error("Stream has no more items and a new connection is required")]
    StreamComplete,
}

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

    pub async fn stream_companies(
        &self,
        timepoint: Option<usize>,
    ) -> Result<StreamConnection, CompaniesHouseStreamingError> {
        match &timepoint {
            Some(t) => log::info!("Connecting to stream with timepoint {t}"),
            None => log::info!("Connecting to stream without timepoint..."),
        };

        let mut request = reqwest::Client::new()
            .get("https://stream.companieshouse.gov.uk/companies")
            .basic_auth(&self.api_key, Option::<&str>::None);

        if let Some(timepoint) = timepoint {
            request = request.query(&[("timepoint", timepoint)]);
        }

        let response = timeout(self.connection_timeout, request.send())
            .await
            .map_err(|_| {
                log::info!("Connection timeout");
                CompaniesHouseStreamingError::ConnectionTimeout
            })?
            .map_err(CompaniesHouseStreamingError::UnknownConnection)?;

        log::info!(status = response.status().as_u16(); "Connection successful");

        Ok(StreamConnection {
            buffer: BytesMut::new(),
            max_chunk_timeout: self.chunk_timeout,
            response,
        })
    }
}

pub struct StreamConnection {
    buffer: BytesMut,
    response: reqwest::Response,
    max_chunk_timeout: Duration,
}

impl StreamConnection {
    pub async fn next(&mut self) -> Result<serde_json::Value, CompaniesHouseStreamingError> {
        loop {
            if let Some((index, _)) = self.buffer.iter().enumerate().find(|(_, c)| **c == b'\n') {
                let left = self.buffer.split_to(index + 1);
                let str = from_utf8(&left)
                    .map_err(CompaniesHouseStreamingError::BadValueEncoding)?
                    .trim();
                if str.is_empty() {
                    log::trace!("Buffer contains empty line");
                } else {
                    log::trace!(length = str.len(); "Buffer contains next stream item");
                    let value: serde_json::Value = serde_json::from_str(str)
                        .map_err(CompaniesHouseStreamingError::BadValueJson)?;
                    return Ok(value);
                }
            }

            log::trace!("Buffer contains no items, reading next response chunk");

            let Some(chunk) = timeout(self.max_chunk_timeout, self.response.chunk())
                .await
                .map_err(|_| {
                    log::info!("Chunk timeout exceeded");
                    CompaniesHouseStreamingError::ChunkTimeout
                })?
                .map_err(CompaniesHouseStreamingError::BadChunk)?
            else {
                return Err(CompaniesHouseStreamingError::StreamComplete);
            };

            log::trace!(bytes = chunk.len(); "Stream chunk received");

            if self.buffer.is_empty() && chunk.len() == 1 && chunk[0] == b'\n' {
                log::info!("Heartbeat chunk received");
            } else {
                self.buffer.extend_from_slice(&chunk);
            }
        }
    }
}

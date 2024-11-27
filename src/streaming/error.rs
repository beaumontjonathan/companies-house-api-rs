use reqwest::StatusCode;
use thiserror::Error;

use super::operation::StreamItem;

#[derive(Debug, Error)]
pub enum CompaniesHouseStreamingConnectionError {
    #[error("Connection timeout exceeded")]
    ConnectionTimeout,
    #[error("Unknown error connecting to stream endpoint")]
    UnknownConnection(reqwest::Error),
    #[error("Not authorised to connect to this stream")]
    Unauthorized,
    #[error("Rate limiting")]
    TooManyRequests,
    #[error("Timepoint specified is invalid or too old")]
    BadTimepoint,
    #[error("Unknown connection response {0}")]
    UnknownResponse(StatusCode),
}

#[derive(Debug, Error)]
pub enum CompaniesHouseStreamingNextError {
    #[error("Stream chunk timeout exceeded")]
    ChunkTimeout,
    #[error("Stream has no more items and a new connection is required")]
    StreamComplete,
    #[error("Unable to read from response body")]
    BadChunk(reqwest::Error),
    #[error("Unable to read utf8 from response body")]
    BadItemEncoding(std::str::Utf8Error),
    #[error("Unable to deserialize next stream item as JSON: {inner}")]
    BadItemJson {
        inner: serde_json::Error,
        text: String,
    },
    #[error("Unable to convert JSON to data type: {inner}")]
    BadItemData {
        inner: serde_json::Error,
        value: StreamItem<serde_json::Value>,
    },
}

use derive_more::derive::Display;
use reqwest::{header::HeaderMap, StatusCode};
use thiserror::Error;

#[derive(Debug, Error, Display)]
#[display("{status_code} {body:?}")]
pub struct UnrecognisedResponse {
    status_code: StatusCode,
    headers: HeaderMap,
    body: UnrecognisedResponseBody,
}

impl UnrecognisedResponse {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status_code = response.status();
        let headers = response.headers().to_owned();

        match response.text().await {
            Err(_) => Self {
                status_code,
                headers,
                body: UnrecognisedResponseBody::Unknown,
            },
            Ok(text) => {
                if text.is_empty() {
                    Self {
                        status_code,
                        headers,
                        body: UnrecognisedResponseBody::Empty,
                    }
                } else {
                    match serde_json::from_str(&text) {
                        Ok(value) => Self {
                            status_code,
                            headers,
                            body: UnrecognisedResponseBody::Json(value),
                        },
                        Err(_) => Self {
                            status_code,
                            headers,
                            body: UnrecognisedResponseBody::Text(text),
                        },
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum UnrecognisedResponseBody {
    Empty,
    Unknown,
    Text(String),
    Json(serde_json::Value),
}

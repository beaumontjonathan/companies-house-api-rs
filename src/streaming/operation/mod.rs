use serde::{de::DeserializeOwned, Deserialize};

pub mod companies;
pub mod filings;

pub trait CompaniesHouseStreamingOperation {
    const ENDPOINT_PATH: &'static str;

    type Data: DeserializeOwned;

    fn endpoint(&self) -> &'static str {
        Self::ENDPOINT_PATH
    }
}

#[derive(Debug, Deserialize)]
pub struct StreamItem<Data> {
    pub data: Data,
    pub event: StreamEvent,
    pub resource_id: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamEvent {
    pub published_at: String,
    pub timepoint: usize,
    pub r#type: StreamEventType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StreamEventType {
    Changed,
    Deleted,
}

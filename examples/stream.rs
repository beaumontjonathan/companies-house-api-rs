use companies_house_api::CompaniesHouseStreamingClient;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Item {
    data: Data,
    event: Event,
}

#[derive(Debug, Deserialize)]
struct Data {
    company_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Event {
    timepoint: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    dotenvy::from_filename(".env.local")?;

    let api_key = env::var("COMPANIES_HOUSE_STREAMING_API_KEY")?;
    let client = CompaniesHouseStreamingClient::new(&api_key);

    let mut latest_timepoint = Some(87268311);

    loop {
        let Ok(mut stream) = client.stream_companies(latest_timepoint).await else {
            continue;
        };

        loop {
            match stream.next().await {
                Err(err) => {
                    log::error!("Stream error: {err}");
                    break;
                }
                Ok(item) => {
                    let item: Item = serde_json::from_value(item)?;
                    latest_timepoint = Some(item.event.timepoint + 1);
                    log::info!(
                        "{}: {}",
                        item.event.timepoint,
                        item.data
                            .company_name
                            .unwrap_or_else(|| "unknown".to_string())
                    );
                }
            }
        }
    }
}

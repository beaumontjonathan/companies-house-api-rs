use companies_house_api::streaming::{
    operation::{companies::StreamCompanies, filings::StreamFilings},
    CompaniesHouseStreamingClient, CompaniesHouseStreamingNextError,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    dotenvy::from_filename(".env.local")?;

    let api_key = std::env::var("COMPANIES_HOUSE_STREAMING_API_KEY")?;
    let client = CompaniesHouseStreamingClient::new(&api_key);

    let mut latest_timepoint = Some(176080588);

    loop {
        let mut stream = match client
            // .stream(StreamCompanies, latest_timepoint.map(|t| t + 1))
            .stream(StreamFilings, latest_timepoint.map(|t| t + 1))
            .await
        {
            Ok(stream) => stream,
            Err(err) => {
                log::error!("Connection failed: {err:?}");
                continue;
            }
        };

        loop {
            match stream.next().await {
                Ok(item) => {
                    latest_timepoint = Some(item.event.timepoint);
                    log::info!(
                        timepoint = item.event.timepoint;
                        "Filing received: {:?}",
                        item.data,
                    );
                }
                Err(CompaniesHouseStreamingNextError::BadItemData { inner, value }) => {
                    log::error!(
                        timepoint = value.event.timepoint;
                        "Error reading JSON data: {inner} {value:?}",
                    );
                    latest_timepoint = Some(value.event.timepoint);
                }
                Err(CompaniesHouseStreamingNextError::BadItemJson { inner, text }) => {
                    anyhow::bail!("Bad json {inner:?} {text}")
                }
                Err(err) => {
                    log::error!("Stream error: {err}");
                    break;
                }
            }
        }
    }
}

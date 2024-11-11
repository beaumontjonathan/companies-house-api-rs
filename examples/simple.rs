use std::env;

use companies_house_api::{
    operation::{
        get_company_profile::GetCompanyProfile,
        get_company_registered_office_address::GetCompanyRegisteredOfficeAddress,
        search_companies::SearchCompanies,
    },
    CompaniesHousePublicDataClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env.local")?;

    let client = CompaniesHousePublicDataClient::new(&env::var("COMPANIES_HOUSE_API_KEY")?)?;

    let data = client
        .send(
            GetCompanyRegisteredOfficeAddress::builder()
                .company_number("00000006")
                .build(),
        )
        .await?;

    println!("{data:#?}");

    let data = client
        .send(
            GetCompanyProfile::builder()
                .company_number("CE033707")
                .build(),
        )
        .await?;

    println!("{data:#?}");

    let data = client
        .send(SearchCompanies::builder().query("b").build())
        .await?;

    println!("{data:#?}");

    Ok(())
}

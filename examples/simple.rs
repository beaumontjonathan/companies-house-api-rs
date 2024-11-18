use minus::dynamic_paging;
use std::env;
use std::fmt::Write;

use companies_house_api::{operation, CompaniesHousePublicDataClient};
use derive_more::derive::Display;

#[derive(Debug, Display)]
enum Operation {
    #[display("Get company profile")]
    GetCompanyProfile,
    #[display("Get company registered office address")]
    GetCompanyRegisteredOfficeAddress,
    #[display("Search companies")]
    SearchCompanies,
    #[display("Search officers")]
    SearchOfficers,
    #[display("Search disqualified officers")]
    SearchDisqualifiedOfficers,
    Quit,
}

fn display_result<T: std::fmt::Debug, E: std::error::Error>(
    result: Result<T, E>,
) -> anyhow::Result<()> {
    let mut pager = minus::Pager::new();
    pager.set_exit_strategy(minus::ExitStrategy::PagerQuit)?;
    match result {
        Ok(inner) => writeln!(pager, "{inner:#?}")?,
        Err(inner) => writeln!(pager, "{inner:#?}")?,
    }
    dynamic_paging(pager)?;
    Ok(())
}

fn get_company_number() -> anyhow::Result<String> {
    Ok(inquire::Text::new("Enter a company number:").prompt()?)
}

fn get_search_term() -> anyhow::Result<String> {
    Ok(inquire::Text::new("Enter a search term:").prompt()?)
}

fn get_page_size() -> anyhow::Result<u8> {
    Ok(inquire::CustomType::new("Enter a page size:").prompt()?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env.local")?;

    let api_key = env::var("COMPANIES_HOUSE_API_KEY")?;
    let client = CompaniesHousePublicDataClient::new(&api_key)?;

    loop {
        let operation = inquire::Select::new(
            "Choose an action:",
            vec![
                Operation::GetCompanyProfile,
                Operation::GetCompanyRegisteredOfficeAddress,
                Operation::SearchCompanies,
                Operation::SearchOfficers,
                Operation::SearchDisqualifiedOfficers,
                Operation::Quit,
            ],
        )
        .prompt()?;

        match operation {
            Operation::GetCompanyProfile => {
                let company_number = get_company_number()?;
                let data = client
                    .send(
                        operation::get_company_profile::GetCompanyProfile::builder()
                            .company_number(company_number)
                            .build(),
                    )
                    .await;
                display_result(data)?;
            }
            Operation::GetCompanyRegisteredOfficeAddress => {
                let company_number = get_company_number()?;
                let data = client
                    .send(
                        operation::get_company_registered_office_address::GetCompanyRegisteredOfficeAddress::builder()
                            .company_number(company_number)
                            .build(),
                    )
                    .await;
                display_result(data)?;
            }
            Operation::SearchCompanies => {
                let search_term = get_search_term()?;
                let data = client
                    .send(
                        operation::search_companies::SearchCompanies::builder()
                            .query(search_term)
                            .build(),
                    )
                    .await;
                display_result(data)?;
            }
            Operation::SearchOfficers => {
                let search_term = get_search_term()?;
                let data = client
                    .send(
                        operation::search_officers::SearchOfficers::builder()
                            .query(search_term)
                            .build(),
                    )
                    .await;
                display_result(data)?;
            }
            Operation::SearchDisqualifiedOfficers => {
                let search_term = get_search_term()?;
                let items_per_page = get_page_size()?;
                let data = client
                .send(
                    operation::search_disqualified_officers::SearchDisqualifiedOfficers::builder()
                        .query(search_term)
                        .items_per_page(Some(items_per_page as usize))
                        .build(),
                )
                .await;

                display_result(data)?;
            }
            Operation::Quit => break,
        }
    }

    Ok(())
}

use crate::types::CompanyProfile;

use super::CompaniesHouseStreamingOperation;

pub struct StreamCompanies;

impl CompaniesHouseStreamingOperation for StreamCompanies {
    const ENDPOINT_PATH: &'static str = "/companies";

    type Data = CompanyProfile;
}

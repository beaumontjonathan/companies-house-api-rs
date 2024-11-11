mod company_profile;
mod company_search;
mod officer_search;

use serde::{Deserialize, Serialize};

pub use company_profile::*;
pub use company_search::*;
pub use officer_search::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Country {
    England,
    Wales,
    Scotland,
    #[serde(rename = "Northern Ireland")]
    NorthernIreland,
    #[serde(rename = "Great Britain")]
    GreatBritain,
    #[serde(rename = "United Kingdom")]
    UnitedKingdom,
    #[serde(rename = "Not specified")]
    NotSpecified,
}

/// Registered Office Address
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredOfficeAddress {
    /// Setting this to true confirms that the new registered office address is an appropriate address as outlined in section 86(2) of the Companies Act 2006.
    pub accept_appropriate_office_address_statement: Option<bool>,
    /// The first line of the address.
    pub address_line_1: String,
    /// The second line of the address.
    pub address_line_2: Option<String>,
    /// The country.
    pub country: Country,
    /// The locality e.g London.
    pub locality: String,
    /// The postal code e.g CF14 3UZ.
    pub postal_code: Option<String>,
    /// The property name or number.
    pub premises: Option<String>,
    /// The region e.g Surrey.
    pub region: Option<String>,
}

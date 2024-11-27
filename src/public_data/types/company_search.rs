use serde::{Deserialize, Serialize};

use super::{shared::Date, OtherString};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearch {
    /// The results of the completed search.
    pub items: Vec<CompanySearchItem>,
    /// The number of search items returned per page.
    pub items_per_page: Option<usize>,
    /// The index into the entire result set that this result page starts.
    pub start_index: Option<usize>,
    /// The number of further search results available for the current search.
    pub total_results: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearchItem {
    pub address: Option<CompanySearchItemAddress>,
    /// A single line address. This will be the address that matched within the indexed document or the primary address otherwise (as returned by the `address` member).
    pub address_snippet: Option<String>,
    /// The company registration / incorporation number of the company.
    pub company_number: String,
    pub company_status: Option<CompanySearchItemCompanyStatus>,
    pub company_type: CompanyStatusItemCompanyType,
    /// The date the company ended.
    pub date_of_cessation: Option<Date>,
    /// The date the company was created.
    pub date_of_creation: Option<Date>,
    /// The result description.
    pub description: Option<String>,
    /// An array of enumeration types that make up the search description.
    pub description_identifier: Option<Vec<Option<CompanySearchItemDescriptionIdentifier>>>,
    pub matches: Option<CompanySearchItemMatches>,
    /// Summary information for the result showing additional details that have matched.
    pub snippet: Option<String>,
    /// The title of the search result.
    pub title: String,
}

/// The address of the company's registered office.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearchItemAddress {
    /// The first line of the address.
    pub address_line_1: Option<String>,
    /// The second line of the address.
    pub address_line_2: Option<String>,
    /// The care of name.
    pub care_of: Option<String>,
    /// The country.
    pub country: Option<OtherString<CompanySearchItemAddressCountry>>,
    /// The locality e.g London.
    pub locality: Option<String>,
    /// The post-office box number.
    pub po_box: Option<String>,
    /// The postal code e.g CF14 3UZ.
    pub postal_code: Option<String>,
    /// The region e.g Surrey.
    pub region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CompanySearchItemAddressCountry {
    Wales,
    England,
    Scotland,
    #[serde(rename = "Great Britain")]
    GreatBritain,
    #[serde(rename = "Not specified")]
    NotSpecified,
    #[serde(rename = "United Kingdom")]
    UnitedKingdom,
    #[serde(rename = "Northern Ireland")]
    NorthernIreland,
    #[serde(rename = "Isle Of Man")]
    IsleOfMan,
    #[serde(rename = "Virgin Islands, British")]
    BritishVirginIslands,
}

/// The company status.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanySearchItemCompanyStatus {
    Active,
    Dissolved,
    Liquidation,
    Receivership,
    Administration,
    VoluntaryArrangement,
    ConvertedClosed,
    InsolvencyProceedings,
    Registered,
    Removed,
}

/// The company type.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyStatusItemCompanyType {
    PrivateUnlimited,
    Ltd,
    Plc,
    OldPublicCompany,
    PrivateLimitedGuarantNscLimitedExemption,
    LimitedPartnership,
    PrivateLimitedGuarantNsc,
    ConvertedOrClosed,
    PrivateUnlimitedNsc,
    #[serde(rename = "private-limited-shares-section-30-exemption")]
    PrivateLimitedSharesSection30Exemption,
    AssuranceCompany,
    OverseaCompany,
    Eeig,
    IcvcSecurities,
    IcvcWarrant,
    IcvcUmbrella,
    IndustrialAndProvidentSociety,
    NorthernIreland,
    NorthernIrelandOther,
    RoyalCharter,
    InvestmentCompanyWithVariableCapital,
    UnregisteredCompany,
    Llp,
    Other,
    EuropeanPublicLimitedLiabilityCompanySe,
    RegisteredOverseasEntity,
    /// Almost all company data is blank for this type
    CharitableIncorporatedOrganisation,
}

/// The company status.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanySearchItemDescriptionIdentifier {
    IncorporatedOn,
    RegisteredOn,
    FormedOn,
    DissolvedOn,
    ConvertedClosedOn,
    ClosedOn,
    Closed,
    FirstUkEstablishmentOpenedOn,
    OpenedOn,
    VoluntaryArrangement,
    Receivership,
    InsolvencyProceedings,
    Liquidation,
    Administration,
    Registered,
    Removed,
    RegisteredExternally,
}

/// A list of members and arrays of character offset defining substrings that matched the search terms.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearchItemMatches {
    /// An array of character offset into the `address_snippet` string. These always occur in pairs and define the start and end of substrings in the member `address_snippet` that matched the search terms.
    pub address_snippet: Option<Vec<usize>>,
    /// An array of character offset into the `snippet` string. These always occur in pairs and define the start and end of substrings in the member snippet that matched the search terms. The first character of the string is index 1.
    pub snippet: Option<Vec<usize>>,
    /// An array of character offset into the `title` string. These always occur in pairs and define the start and end of substrings in the member `title` that matched the search terms. The first character of the string is index 1.
    pub title: Option<Vec<usize>>,
}

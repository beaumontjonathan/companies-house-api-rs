use serde::{Deserialize, Serialize};

type Date = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearch {
    /// The results of the completed search.
    items: Vec<CompanySearchItem>,
    /// The number of search items returned per page.
    items_per_page: Option<usize>,
    /// The index into the entire result set that this result page starts.
    start_index: Option<usize>,
    /// The number of further search results available for the current search.
    total_results: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearchItem {
    address: Option<CompanySearchItemAddress>,
    /// A single line address. This will be the address that matched within the indexed document or the primary address otherwise (as returned by the `address` member).
    address_snippet: Option<String>,
    /// The company registration / incorporation number of the company.
    company_number: String,
    company_status: Option<CompanySearchItemCompanyStatus>,
    company_type: CompanyStatusItemCompanyType,
    /// The date the company ended.
    date_of_cessation: Option<Date>,
    /// The date the company was created.
    date_of_creation: Option<Date>,
    /// The result description.
    description: Option<String>,
    /// An array of enumeration types that make up the search description.
    description_identifier: Option<Vec<Option<CompanySearchItemDescriptionIdentifier>>>,
    matches: Option<CompanySearchItemMatches>,
    /// Summary information for the result showing additional details that have matched.
    snippet: Option<String>,
    /// The title of the search result.
    title: String,
}

/// The address of the company's registered office.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySearchItemAddress {
    /// The first line of the address.
    address_line_1: Option<String>,
    /// The second line of the address.
    address_line_2: Option<String>,
    /// The care of name.
    care_of: Option<String>,
    /// The country.
    country: Option<OtherString<CompanySearchItemAddressCountry>>,
    /// The locality e.g London.
    locality: Option<String>,
    /// The post-office box number.
    po_box: Option<String>,
    /// The postal code e.g CF14 3UZ.
    postal_code: Option<String>,
    /// The region e.g Surrey.
    region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
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
    Other(String),
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
    address_snippet: Option<Vec<usize>>,
    /// An array of character offset into the `snippet` string. These always occur in pairs and define the start and end of substrings in the member snippet that matched the search terms. The first character of the string is index 1.
    snippet: Option<Vec<usize>>,
    /// An array of character offset into the `title` string. These always occur in pairs and define the start and end of substrings in the member `title` that matched the search terms. The first character of the string is index 1.
    title: Option<Vec<usize>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum OtherString<T> {
    Value(T),
    Other(String),
}

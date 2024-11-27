use serde::{Deserialize, Serialize};

use super::shared::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearch {
    /// The results of the completed search.
    pub items: Option<Vec<DisqualifiedOfficerSearchItem>>,
    /// The number of search items returned per page.
    pub items_per_page: Option<usize>,
    /// The index into the entire result set that this result page starts.
    pub start_index: Option<usize>,
    /// The number of further search results available for the current search.
    pub total_results: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearchItem {
    /// The address of the disqualified officer as provided by the disqualifying authority.
    pub address: DisqualifiedOfficerSearchItemAddress,
    /// A single line address. This will be the address that matched within the indexed
    /// document or the primary address otherwise (as returned by the `address` member).
    pub address_snippet: String,
    /// The disqualified officer's date of birth.
    pub date_of_birth: Option<Date>,
    /// The result description.
    pub description: String,
    pub description_identifiers: Option<Vec<DisqualifiedOfficerSearchItemDescriptionIdentifiers>>,
    pub matches: Option<DisqualifiedOfficerSearchItemMatches>,
    /// Summary information for the result showing additional details that have matched.
    pub snippet: Option<String>,
    /// The title of the search result.
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearchItemAddress {
    /// The first line of the address.
    pub address_line_1: Option<String>,
    /// The second line of the address.
    pub address_line_2: Option<String>,
    /// The country. For example UK.
    pub country: Option<String>,
    /// The locality. For example London.
    pub locality: Option<String>,
    /// The postal code. For example CF14 3UZ.
    pub postal_code: Option<String>,
    /// The property name or number.
    pub premises: Option<String>,
    /// The region. For example Surrey.
    pub region: Option<String>,
}

/// An array of enumeration types that make up the search description.
/// See `disqualified_officer_search_description` section in [search descriptions](https://github.com/companieshouse/api-enumerations/blob/master/search_descriptions_raw.yaml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DisqualifiedOfficerSearchItemDescriptionIdentifiers {
    BornOn,
}

/// A list of members and arrays of character offset defining substrings that matched the search terms.
#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearchItemMatches {
    /// An array of character offset into the `address_snippet` string. These always occur in pairs and define the start and end of substrings in the member `address_snippet` that matched the search terms.
    pub address_snippet: Option<Vec<usize>>,
    /// An array of character offset into the `snippet` string. These always occur in pairs and define the start and end of substrings in the member snippet that matched the search terms. The first character of the string is index 1.
    pub snippet: Option<Vec<usize>>,
    /// An array of character offset into the `title` string. These always occur in pairs and define the start and end of substrings in the member `title` that matched the search terms. The first character of the string is index 1.
    pub title: Option<Vec<usize>>,
}

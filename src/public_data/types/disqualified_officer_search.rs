use serde::{Deserialize, Serialize};

type Date = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearch {
    /// The results of the completed search.
    items: Option<Vec<DisqualifiedOfficerSearchItem>>,
    /// The number of search items returned per page.
    items_per_page: Option<usize>,
    /// The index into the entire result set that this result page starts.
    start_index: Option<usize>,
    /// The number of further search results available for the current search.
    total_results: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearchItem {
    /// The address of the disqualified officer as provided by the disqualifying authority.
    address: DisqualifiedOfficerSearchItemAddress,
    /// A single line address. This will be the address that matched within the indexed
    /// document or the primary address otherwise (as returned by the `address` member).
    address_snippet: String,
    /// The disqualified officer's date of birth.
    date_of_birth: Option<Date>,
    /// The result description.
    description: String,
    description_identifiers: Option<Vec<DisqualifiedOfficerSearchItemDescriptionIdentifiers>>,
    matches: Option<DisqualifiedOfficerSearchItemMatches>,
    /// Summary information for the result showing additional details that have matched.
    snippet: Option<String>,
    /// The title of the search result.
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisqualifiedOfficerSearchItemAddress {
    /// The first line of the address.
    address_line_1: Option<String>,
    /// The second line of the address.
    address_line_2: Option<String>,
    /// The country. For example UK.
    country: Option<String>,
    /// The locality. For example London.
    locality: Option<String>,
    /// The postal code. For example CF14 3UZ.
    postal_code: Option<String>,
    /// The property name or number.
    premises: Option<String>,
    /// The region. For example Surrey.
    region: Option<String>,
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
    address_snippet: Option<Vec<usize>>,
    /// An array of character offset into the `snippet` string. These always occur in pairs and define the start and end of substrings in the member snippet that matched the search terms. The first character of the string is index 1.
    snippet: Option<Vec<usize>>,
    /// An array of character offset into the `title` string. These always occur in pairs and define the start and end of substrings in the member `title` that matched the search terms. The first character of the string is index 1.
    title: Option<Vec<usize>>,
}

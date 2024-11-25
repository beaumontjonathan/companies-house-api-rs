use serde::Deserialize;

use super::CompaniesHouseStreamingOperation;

pub struct StreamFilings;

impl CompaniesHouseStreamingOperation for StreamFilings {
    const ENDPOINT_PATH: &'static str = "/filings";

    type Data = FilingHistory;
}

pub type Date = String;

#[derive(Debug, Deserialize)]
pub struct FilingHistory {
    pub annotations: Option<Vec<FilingHistoryAnnotation>>,
    pub associated_filings: Option<Vec<FilingHistoryAssociatedFiling>>,
    /// The barcode of the document.
    pub barcode: Option<String>,
    pub category: FilingHistoryCategory,
    /// The date the filing was processed.
    pub date: Date,
    /// A description of the filing.
    /// For enumeration mappings please see [here](https://github.com/companieshouse/api-enumerations/blob/master/filing_history_descriptions.yml).
    pub description: String,
    pub links: Option<FilingHistoryLinks>,
    /// Number of pages within the PDF document `links.document_metadata`.
    pub pages: Option<usize>,
    /// If true, indicates this is a paper filing.
    pub paper_filed: Option<bool>,
    pub resolutions: Option<Vec<FilingHistoryResolution>>,
    /// The sub-category of the document filed.
    pub subcategory: Option<FilingHistorySubCategory>,
    /// The transaction ID of the filing.
    pub transaction_id: String,
    /// type
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct FilingHistoryAnnotation {
    /// The annotation text.
    pub annotation: Option<String>,
    /// The date the annotation was added.
    pub date: String,
    /// A description of the annotation.
    /// For enumeration mappings please see [here](https://github.com/companieshouse/api-enumerations/blob/master/filing_history_descriptions.yml).
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct FilingHistoryAssociatedFiling {
    /// The date the associated filing was processed.
    pub date: String,
    /// A description of the associated filing.
    /// For enumeration mappings please see [here](https://github.com/companieshouse/api-enumerations/blob/master/filing_history_descriptions.yml).
    pub description: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilingHistoryCategory {
    Accounts,
    Address,
    AnnualReturn,
    Capital,
    ConfirmationStatement,
    ChangeOfName,
    Incorporation,
    Liquidation,
    Miscellaneous,
    Mortgage,
    Officers,
    PersonsWithSignificantControl,
    Resolution,
}

#[derive(Debug, Deserialize)]
pub struct FilingHistoryLinks {
    /// Link to the document metadata associated with this filing history item. See the Document API documentation for more details.
    pub document_metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FilingHistoryResolution {
    pub category: FilingHistoryResolutionCategory,
    /// A description of the associated filing.
    /// /// For enumeration mappings please see [here](https://github.com/companieshouse/api-enumerations/blob/master/filing_history_descriptions.yml).
    pub description: String,
    /// The document id of the resolution.
    pub document_id: Option<String>,
    /// The date the resolution was processed.
    pub receive_date: Date,
    pub subcategory: FilingHistorySubCategory,
    /// The type of the associated filing.
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilingHistoryResolutionCategory {
    Miscellaneous,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilingHistorySubCategory {
    Resolution,
}

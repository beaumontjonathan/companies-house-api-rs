//! https://developer-specs.company-information.service.gov.uk/companies-house-public-data-api/resources/companyprofile

use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_number_from_string, deserialize_option_number_from_string};

use super::Country;

type Date = String;

/// Company Profile
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfile {
    /// Company accounts information.
    accounts: Option<CompanyProfileAccounts>,
    annual_return: Option<CompanyProfileAnnualReturn>,
    branch_company_details: Option<CompanyProfileBranchCompanyDetails>,
    /// Flag indicating whether this company can file.
    can_file: bool,
    /// The name of the company.
    company_name: String,
    /// The number of the company.
    company_number: String,
    company_status: Option<CompanyProfileCompanyStatus>,
    company_status_detail: Option<CompanyProfileCompanyStatusDetail>,
    confirmation_statement: Option<CompanyProfileConfirmationStatement>,
    /// The date which the company was converted/closed, dissolved or removed. Please refer to company status to determine which.
    date_of_cessation: Option<Date>,
    /// The date when the company was created.
    date_of_creation: Option<Date>,
    /// The number given by an external registration body.
    external_registration_number: Option<String>,
    foreign_company_details: Option<CompanyProfileForeignCompanyDetails>,
    /// Deprecated. Please use `links.insolvency`.
    has_been_liquidated: Option<bool>,
    /// Deprecated. Please use `links.charges`.
    has_charges: Option<bool>,
    /// Deprecated. Please use `links.insolvency`.
    has_insolvency_history: Option<bool>,
    /// Deprecated. Please use `subtype`.
    is_community_interest_company: Option<bool>,
    jurisdiction: Option<CompanyProfileJurisdiction>,
    /// The date of last full members list update.
    last_full_members_list_date: Option<Date>,
    links: CompanyProfileLinks,
    partial_data_available: Option<CompanyProfilePartialDataAvailable>,
    previous_company_names: Option<Vec<CompanyProfilePreviousCompanyName>>,
    registered_office_address: Option<CompanyProfileRegisteredOfficeAddress>,
    /// Flag indicating registered office address as been replaced.
    registered_office_is_in_dispute: Option<bool>,
    service_address: Option<CompanyProfileServiceAddress>,
    /// SIC codes for this company.
    sic_codes: Option<Vec<String>>,
    subtype: Option<CompanyProfileSubtype>,
    /// The total count of super secure managing officers for a `registered-overseas-entity`.
    super_secure_managing_officer_count: Option<u8>,
    r#type: CompanyProfileType,
    /// Flag indicating whether post can be delivered to the registered office.
    undeliverable_registered_office_address: Option<bool>,
}

/// Company accounts information.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileAccounts {
    /// The Accounting Reference Date (ARD) of the company.
    accounting_reference_date: CompanyProfileAccountsAccountingReferenceDate,
    last_accounts: Option<CompanyProfileAccountsLastAccounts>,
    next_account: Option<CompanyProfileAccountsNextAccounts>,
    /// Deprecated. Please use `accounts.next_accounts.due_on`.
    next_due: Option<Date>,
    /// Deprecated. Please use `accounts.next_accounts.period_end_on`.
    next_made_up_to: Option<Date>,
    /// Deprecated. Please use `accounts.next_accounts.overdue`.
    overdue: Option<bool>,
}

/// The Accounting Reference Date (ARD) of the company.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileAccountsAccountingReferenceDate {
    /// The Accounting Reference Date (ARD) day.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    day: u8,
    /// The Accounting Reference Date (ARD) month.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    month: u8,
}

/// The last company accounts filed.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileAccountsLastAccounts {
    /// Deprecated. Please use `period_end_on`.
    made_up_to: Date,
    /// The last day of the most recently filed accounting period.
    period_end_on: Option<Date>,
    /// The first day of the most recently filed accounting period.
    period_start_on: Option<Date>,
    /// The type of the last company accounts filed.
    r#type: CompanyProfileAccountsLastAccountsType,
}

/// The type of the last company accounts filed.
/// See `account_type` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileAccountsLastAccountsType {
    Null,
    Full,
    Small,
    Medium,
    Group,
    Dormant,
    Interim,
    Initial,
    TotalExemptionFull,
    TotalExemptionSmall,
    PartialExemption,
    AuditExemptionSubsidiary,
    FilingExemptionSubsidiary,
    MicroEntity,
    NoAccountsTypeAvailable,
    AuditedAbridged,
    UnauditedAbridged,
}

/// The next company accounts filed.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileAccountsNextAccounts {
    /// The date the next company accounts are due
    due_on: Option<Date>,
    /// Flag indicating if the company accounts are overdue.
    overdue: Option<bool>,
    /// The last day of the next accounting period to be filed.
    period_end_on: Option<Date>,
    /// The first day of the next accounting period to be filed.
    period_start_on: Option<Date>,
}

/// Annual return information. This member is only returned if a confirmation statement has not be filed.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileAnnualReturn {
    /// The date the last annual return was made up to.
    last_made_up_to: Option<Date>,
    /// The date the next annual return is due. This member will only be returned if a confirmation statement has not been filed and the date is before 28th July 2016, otherwise refer to `confirmation_statement.next_due`.
    next_due: Option<Date>,
    /// The date the next annual return should be made up to. This member will only be returned if a confirmation statement has not been filed and the date is before 30th July 2016, otherwise refer to `confirmation_statement.next_made_up_to`.
    next_made_up_to: Option<Date>,
    /// Flag indicating if the annual return is overdue.
    overdue: Option<bool>,
}

/// UK branch of a foreign company.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileBranchCompanyDetails {
    /// Type of business undertaken by the UK establishment.
    business_activity: Option<String>,
    /// Parent company name.
    parent_company_name: Option<String>,
    /// Parent company number.
    parent_company_number: Option<String>,
}

/// The status of the company.
/// See `company_status` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileCompanyStatus {
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
    Closed,
    Open,
}

/// Extra details about the status of the company.
/// See `company_status_detail` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileCompanyStatusDetail {
    TransferredFromUk,
    ActiveProposalToStrikeOff,
    PetitionToRestoreDissolved,
    TransformedToSe,
    ConvertedToPlc,
}

/// Confirmation statement information (N.B. refers to the Annual Statement where type is registered-overseas-entity).
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileConfirmationStatement {
    /// The date to which the company last made a confirmation statement.
    last_made_up_to: Option<Date>,
    /// The date by which the next confirmation statement must be received.
    next_due: Date,
    /// The date to which the company must next make a confirmation statement.
    next_made_up_to: Date,
    /// Flag indicating if the confirmation statement is overdue.
    overdue: Option<bool>,
}

/// Foreign company details.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileForeignCompanyDetails {
    accounting_requirement: Option<CompanyProfileForeignCompanyDetailsAccountingRequirement>,
    accounts: Option<CompanyProfileForeignCompanyDetailsAccounts>,
    /// Type of business undertaken by the company.
    business_activity: Option<String>,
    /// Legal form of the company in the country of incorporation.
    company_type: Option<String>,
    /// Law governing the company in country of incorporation.
    governed_by: Option<String>,
    /// Is it a financial or credit institution.
    is_a_credit_finance_institution: Option<bool>,
    originating_registry: Option<CompanyProfileForeignCompanyDetailsOriginatingRegistry>,
    /// Registration number in company of incorporation.
    registration_number: Option<String>,
}

/// Accounts requirement.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileForeignCompanyDetailsAccountingRequirement {
    foreign_account_type:
        Option<CompanyProfileForeignCompanyDetailsAccountingRequirementForeignAccountType>,
    terms_of_account_publication:
        Option<CompanyProfileForeignCompanyDetailsAccountingRequirementTermsOfAccountPublication>,
}

/// Type of accounting requirement that applies.
/// See `foreign_account_type` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileForeignCompanyDetailsAccountingRequirementForeignAccountType {
    AccountingRequirementsOfOriginatingCountryApply,
    AccountingRequirementsOfOriginatingCountryDoNotApply,
}

/// Type of accounting requirement that applies.
/// See `terms_of_account_publication` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileForeignCompanyDetailsAccountingRequirementTermsOfAccountPublication {
    AccountsPublicationDateSuppliedByCompany,
    AccountingPublicationDateDoesNotNeedToBeSuppliedByCompany,
    AccountingReferenceDateAllocatedByCompaniesHouse,
}

/// Foreign company account information.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileForeignCompanyDetailsAccounts {
    /// Date account period starts under parent law.
    account_period_from: Option<CompanyProfileForeignCompanyDetailsAccountsAccountPeriod>,
    /// Date account period ends under parent law.
    account_period_to: Option<CompanyProfileForeignCompanyDetailsAccountsAccountPeriod>,
    must_file_within: Option<CompanyProfileForeignCompanyDetailsAccountsMustFileWithin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileForeignCompanyDetailsAccountsAccountPeriod {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    day: Option<u8>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    month: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileForeignCompanyDetailsAccountsMustFileWithin {
    /// Number of months within which to file.
    months: Option<u8>,
}

/// Company origin information.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileForeignCompanyDetailsOriginatingRegistry {
    /// Country in which company was incorporated.
    country: Option<String>,
    /// Identity of register in country of incorporation.
    name: Option<String>,
}

/// The jurisdiction specifies the political body responsible for the company.
/// See `jurisdiction` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileJurisdiction {
    EnglandWales,
    Wales,
    Scotland,
    NorthernIreland,
    EuropeanUnion,
    UnitedKingdom,
    England,
    #[serde(rename = "noneu")]
    NonEu,
}

/// A set of URLs related to the resource, including self.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileLinks {}

/// Returned if Companies House is not the primary source of data for this company.
/// See `partial_data_available` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfilePartialDataAvailable {
    FullDataAvailableFromFinancialConductAuthority,
    FullDataAvailableFromDepartmentOfTheEconomy,
    FullDataAvailableFromTheCompany,
}

/// A previous name of this company.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfilePreviousCompanyName {
    /// The date on which the company name ceased.
    ceased_on: Date,
    /// The date from which the company name was effective.
    effective_from: Date,
    /// The previous company name.
    name: String,
}

/// The address of the company's registered office.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileRegisteredOfficeAddress {
    /// The first line of the address.
    address_line_1: Option<String>,
    /// The second line of the address.
    address_line_2: Option<String>,
    /// The care of name.
    care_of: Option<String>,
    /// The country.
    country: Option<Country>,
    /// The locality e.g London.
    locality: Option<String>,
    /// The post-office box number.
    po_box: Option<String>,
    /// The postal code e.g CF14 3UZ.
    postal_code: Option<String>,
    /// The property name or number.
    premises: Option<String>,
    /// The region e.g Surrey.
    region: Option<String>,
}

/// The correspondence address of a Registered overseas entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyProfileServiceAddress {
    /// The first line of the address.
    address_line_1: Option<String>,
    /// The second line of the address.
    address_line_2: Option<String>,
    /// The care of name.
    care_of: Option<String>,
    /// The country e.g United Kingdom.
    country: Option<String>,
    /// The locality e.g London.
    locality: Option<String>,
    /// The post-office box number.
    po_box: Option<String>,
    /// The postal code e.g CF14 3UZ.
    postal_code: Option<String>,
    /// The region e.g Surrey.
    region: Option<String>,
}

/// The subtype of the company.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileSubtype {
    CommunityInterestCompany,
    PrivateFundLimitedPartnership,
}

/// The type of the company.
/// See `company_type` section in [enumeration mappings](https://github.com/companieshouse/api-enumerations/blob/master/constants.yml).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompanyProfileType {
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
    ProtectedCellCompany,
    AssuranceCompany,
    OverseaCompany,
    Eeig,
    IcvcSecurities,
    IcvcWarrant,
    IcvcUmbrella,
    RegisteredSocietyNonJurisdictional,
    IndustrialAndProvidentSociety,
    NorthernIreland,
    NorthernIrelandOther,
    RoyalCharter,
    InvestmentCompanyWithVariableCapital,
    UnregisteredCompany,
    Llp,
    Other,
    EuropeanPublicLimitedLiabilityCompanySe,
    UkEstablishment,
    ScottishPartnership,
    CharitableIncorporatedOrganisation,
    ScottishCharitableIncorporatedOrganisation,
    FurtherEducationOrSixthFormCollegeCorporation,
    RegisteredOverseasEntity,
}

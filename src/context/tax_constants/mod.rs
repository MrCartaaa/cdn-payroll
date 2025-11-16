//! Initialize Tax Constants by Year (see [Version](../tax_constants/enum.Version.html)) for Payroll Tax Calculations.

pub mod canada_pension_plan;
pub mod claim_codes;
pub mod ei_rates_and_amounts;
pub mod income_threshold_and_constants;
pub mod other_rates_and_amounts;

use std::error::Error;

use canada_pension_plan::*;
use claim_codes::federal_claim_codes::*;
use claim_codes::provincial_claim_codes::*;
use ei_rates_and_amounts::*;
use income_threshold_and_constants::*;
use other_rates_and_amounts::*;

/** TaxConstants: used to Initialize Constants by Year.
*
* CSV files are pulled directly from the [CRA website](https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas.html) and represented here.
*/
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct TaxConstants {
    pub version: Version,
    pub prov: Province,
    pub fed: Federal,
    pub CPP: CPPCtx,
    pub EI: EI_RA,
}

impl TaxConstants {
    /** Create New TaxConstants.
     */
    #[allow(non_snake_case)]
    pub fn new(version: Version, province: ProvinceKey) -> Result<Self, Box<dyn Error>> {
        let prov = Province::init(&version, &province)?;

        let fed = Federal::init(&version)?;

        let EI = EI_RA::init(&version, &province)?;

        let CPP = CPPCtx::new(&version, &province)?;

        Ok(Self {
            version,
            fed,
            prov,
            CPP,
            EI,
        })
    }
}

/// TaxConstants Version
///
/// This directs TaxConstants to read the correct CRA files
#[derive(Debug)]
pub enum Version {
    V2025_1,
}

/// Federal Constants
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Federal {
    pub ORA: OtherRatesAndAmounts,
    pub CC: Vec<FederalClaimCode>,
    pub RITC: FedRITC,
}

impl FedORAGetter for Federal {}
impl FederalClaimCodesGetter for Federal {}
impl FedRITCGetter for Federal {}

impl Federal {
    pub fn init(version: &Version) -> Result<Federal, Box<dyn Error>> {
        Ok(Self {
            ORA: Self::init_otr(&version)?,
            CC: Self::init_cc(&version)?,
            RITC: Self::init_ritc(&version)?,
        })
    }
}

/// ## Provincial Constants
///
/// when the [Context](../struct.Context.html) is initialized, it uses the province key to identify
/// which tax tables to use; those tables are represented here with the related fields.
///
/// ### Fields:
///
/// ORA: Other Rates and Amounts
///
/// CC: Claim Codes
///
/// RITC: Rates, Income Thresholds and Constants
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Province {
    pub prov: ProvinceKey,
    pub ORA: ORA,
    pub CC: Vec<ProvincialClaimCode>,
    pub RITC: ProvRITC,
}

impl ProvORAGetter for Province {}
impl ProvincialClaimCodesGetter for Province {}
impl ProvRITCGetter for Province {}

impl Province {
    // Initialize Provincial Constants
    pub fn init(version: &Version, prov: &ProvinceKey) -> Result<Province, Box<dyn Error>> {
        Ok(Self {
            prov: prov.to_owned(),
            ORA: Self::init_otr(&version, &prov)?,
            CC: Self::init_cc(&version, &prov)?,
            RITC: Self::init_ritc(&version, &prov)?,
        })
    }
}

// Province Enum
//
// This directs TaxConstants to get the correct provincal constants defined by the user,
// upstream
#[derive(Debug, Clone)]
pub enum ProvinceKey {
    AB,
    BC,
    MB,
    NB,
    NL,
    NS,
    NT,
    NU,
    ON,
    QC,
    PE,
    SK,
    YT,
}

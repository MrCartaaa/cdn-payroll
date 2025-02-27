//! Context: used to Initialize Constants by Year for Payroll Tax Calculations.

// this should be deleted when migrating context is finished;
pub const EI_MAX_CONTRIBUTIONS: f64 = 1077.48;
pub const CPP_MAX_CONTRIBUTIONS: f64 = 3356.1;
pub const INCOME_THRESHOLD_4: f64 = 177882.0;
pub const INCOME_THRESHOLD_5: f64 = 253414.0;
pub const MINIMUM_BASIC_AMT: f64 = 16129.0;
pub const MAXIMUM_BASIC_AMT: f64 = 14538.0;

pub mod canada_pension_plan;
pub mod income_threshold_and_constants;
pub mod ei_rates_and_amounts;
pub mod other_rates_and_amounts;
pub mod claim_codes;

use std::error::Error;

use income_threshold_and_constants::*;
use canada_pension_plan::cpp_contribution_rates_and_amounts::*;
use canada_pension_plan::base_cpp_rates_and_amounts::*;
use canada_pension_plan::first_additional_cpp_rates_and_amounts::*;
use canada_pension_plan::second_additional_cpp_rates_and_amounts::*;
use ei_rates_and_amounts::*;
use claim_codes::federal_claim_codes::*;
use claim_codes::provincial_claim_codes::*;
use other_rates_and_amounts::*;

/** Context: used to Initialize Constants by Year.
*
* CSV files are pulled directly from the [CRA website](https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas.html) and represented here.
*/ 
#[derive(Debug)]
#[allow(non_snake_case)]
 pub struct Context {
    pub version: Version,
    pub ITC: IncomeThresholdAndConstants,
    pub ORA: OtherRatesAndAmounts,
    pub CC: CCCtx,
    pub CPP: CPPCtx,
    pub EIContRate: EmploymentInsuranceRatesAndAmounts,
 }

 impl Context {

    /** Create New Context.
    */
    #[allow(non_snake_case)]
     pub fn new(version: Version) -> Result<Self, Box< dyn Error>> {
        let ITC = IncomeThresholdAndConstants::init(&version)?;
        let ORA = OtherRatesAndAmounts::init(&version)?;

        let FCC = FederalClaimCodes::init(&version)?;
        let ONCC = ProvincialClaimCodes::init_on(&version)?;

        let CPPContRate = CanadaPensionPlanContributionRatesAndAmounts::init(&version)?;
        let BaseCPPRate = BaseCanadaPensionPlanRatesAndAmounts::init(&version)?;
        let CPPFAddntlRate = FirstAdditionalCanadaPensionPlanRatesAndAmounts::init(&version)?;
        let CPPSAddntlRate = SecondAdditionalCanadaPensionPlanRatesAndAmounts::init(&version)?;

        let EIContRate = EmploymentInsuranceRatesAndAmounts::init(&version)?;

        Ok(Self {
            version,
            ITC,
            ORA,
            CPP: CPPCtx {
                CPPContRate,
                BaseCPPRate,
                CPPFAddntlRate,
                CPPSAddntlRate,
                },
            CC: CCCtx {
                Federal: FCC.CC,
                ON: ONCC.CC,
            },
            EIContRate,
         })
     }
 }

/// Context for CPP Constants
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CPPCtx {
    pub CPPContRate: CanadaPensionPlanContributionRatesAndAmounts,
    pub BaseCPPRate: BaseCanadaPensionPlanRatesAndAmounts,
    pub CPPFAddntlRate: FirstAdditionalCanadaPensionPlanRatesAndAmounts,
    pub CPPSAddntlRate: SecondAdditionalCanadaPensionPlanRatesAndAmounts,
}

/// Context for Federal and Provincial Claim Codes
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CCCtx {
    pub Federal: Vec<FederalClaimCode>,
    pub ON: Vec<ProvincialClaimCode>,
}

/// Context Version
///
/// This directs Context to read the correct CRA files
#[derive(Debug)]
pub enum Version {
    V2025_1
}


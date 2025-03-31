//! # Contexts (Person's) TaxConstants.
//!
//! This defines the individual's persistant details, as defined by the user.

use dotenv;
use std::error::Error;
mod tax_constants;
pub use tax_constants::*;

/** ## Context
*
* ### Where:
*
*   tax_constants: TaxConstants
*
*   payer_vars: TaxPayerVariables
*
*/
#[allow(non_snake_case)]
pub struct Context {
    pub tax_consts: TaxConstants,
    pub payer_vars: TaxPayerVariables,
}

impl Context {

    /** ## Create a Context
    *
    * ### Arguements:
    *
    *   year_version: Version to init context
    *
    *   province: Province to init context
    *
    *   payer_vars: Variables related to the Tax Payer
    */
    #[allow(non_snake_case)]
    pub fn new(year_version: Version, province: ProvinceKey, payer_vars: Option<TaxPayerVariables>) -> Result<Self, Box<dyn Error>> {
        let tax_consts = TaxConstants::new(year_version, province)?;
        let pv: TaxPayerVariables;
        if payer_vars.is_none() {
            if dotenv::var("ENV").unwrap() != "PRODUCTION" {
                pv = TaxPayerVariables::__test__(&tax_consts)?;
            } else {
                return Err("payer_vars [TaxPayerVariables] are required.".into())
            }
        } else {
            pv = payer_vars.unwrap();
        }
        
        Ok(Self {
            tax_consts,
            payer_vars: pv,
        })
    }
}
/* ## Tax Payer Variables.
*
*   TCP: Total Claim Amount as defined by the Provincial or territorial TD1 Form.
*
*   P: Number of pay periods.
*
*   PR: Number of pay periods left in the year (including the current pay period).
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution).
*
*   PE: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, plus PEYTD
*
*   L: Additional tax deductions for the pay period requested by the employee or pensioner as shown on Form TD1
*
*   F1: Annual deductions such as child care expenses and support payments requested by an employee or pensioner and authorized by a tax services office or tax centre
*
*   HD: Annual deduction for living in a prescribed zone, as shown on Form TD1
*
*   U1: Union dues for the pay period paid to a trade union, an association of public servants, or dues required under the law of a province to a parity or advisory committee or similar body
*
*   F2: Alimony or maintenance payments required by a legal document dated before May 1, 1997, to be payroll-deducted authorized by a tax services office or tax centre
*
*   lives_outside_city_limits: outside Canada and in Canada beyond the limits of any province or territory.
*
*   number_of_disabled_dependants: Number of disabled dependants
*
*   number_of_minor_dependents: Number of dependents under the age of 19
*/
#[allow(non_snake_case)]
pub struct TaxPayerVariables {
    pub TCP: f64,
    pub P: i64,
    pub PR: i64,
    pub PM: i64,
    pub PE: i64,
    pub L: Option<f64>,
    pub F1: Option<f64>,
    pub HD: Option<f64>,
    pub U1: Option<f64>,
    pub F2: Option<f64>,
    pub lives_outside_city_limits: bool,
    pub number_of_disabled_dependants: Option<i64>,
    pub number_of_minor_dependents: Option<i64>,

}

impl TaxPayerVariables {

    #[allow(non_snake_case)]
    pub fn new(TCP: f64, P: i64, PR: i64, PM: i64, PE: i64, L: Option<f64>, F1: Option<f64>, HD: Option<f64>, U1: Option<f64>, F2: Option<f64>,
                lives_outside_city_limits: bool, number_of_disabled_dependants: Option<i64>, number_of_minor_dependents: Option<i64>) -> Self {
        TaxPayerVariables {
            TCP,
            P,
            PR,
            PM,
            PE,
            L,
            F1,
            HD,
            U1,
            F2,
            lives_outside_city_limits,
            number_of_disabled_dependants,
            number_of_minor_dependents,
        }
    }

    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub fn __test__(tax_constants: &TaxConstants) -> Result<Self, Box<dyn Error>> {
        let TCP: f64  = tax_constants.prov.ORA.get_basic_amount_value().unwrap();

        Ok(TaxPayerVariables::new(
            TCP,
            52,
            52,
            12,
            0,
            None,
            None,
            None,
            None,
            None,
            false,
            None,
            None,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_payer_variables_test_fn_is_ok() {
        let result = TaxConstants::new(Version::V2025_1, ProvinceKey::ON); 
        assert!(result.is_ok());
        let tax_constants = result.unwrap();

        let result = TaxPayerVariables::__test__(&tax_constants);
        assert!(result.is_ok());
        let payer = result.unwrap();

        assert_eq!(payer.TCP, 12747.0);

    }

    #[test]
    fn test_tax_payer_variables_test_fn_is_err() {
        let result = TaxConstants::new(Version::V2025_1, ProvinceKey::YT); 
        assert!(result.is_err());
    }
}

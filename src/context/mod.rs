//! # Contexts (Person's) TaxConstants.
//!
//! This defines the individual's persistant details, as defined by the user.

use std::error::Error;
mod tax_constants;
pub use tax_constants::*;

/** ## Context
*
* ### Where:
*
*   tax_constants: TaxConstants.
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
pub struct Context {
    pub tax_constants: TaxConstants,
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

impl Context {

    /** ## Create a Context
    *
    * ### Arguements:
    *
    *   year_version: Version to init context
    *
    *   province: Province to init context
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
    pub fn new(year_version: Version, province: ProvinceKey, TCP: f64, P: i64, PR: i64, PM: i64, PE: i64, L: Option<f64>, F1: Option<f64>, HD: Option<f64>, U1: Option<f64>, F2: Option<f64>, lives_outside_city_limits: bool, number_of_minor_dependents: Option<i64>, number_of_disabled_dependants: Option<i64>) -> Result<Self, Box<dyn Error>> {
        let tax_constants = TaxConstants::new(year_version, province)?;
        Ok(Self {
            tax_constants,
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
        })
    }
}

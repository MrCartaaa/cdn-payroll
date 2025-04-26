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
    pub fn new(
        year_version: Version,
        province: ProvinceKey,
        payer_vars: Option<TaxPayerVariables>,
    ) -> Result<Self, Box<dyn Error>> {
        let tax_consts = TaxConstants::new(year_version, province)?;
        let pv: TaxPayerVariables;
        if payer_vars.is_none() {
            if dotenv::var("ENV").unwrap() != "PRODUCTION" {
                pv = TaxPayerVariables::__test__(&tax_consts, None)?;
            } else {
                return Err("payer_vars [TaxPayerVariables] are required.".into());
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
*   I: Gross remuneration for the pay period.
*
*   This includes overtime earned and paid in the same pay period, pension income, qualified pension income, and taxable benefits, but does not include bonuses, retroactive pay increases, or other non-periodic payments
*
*   TCP: Total Claim Amount as defined by the Provincial or territorial TD1 Form.
*
*   P: Number of pay periods.
*
*   PR: Number of pay periods left in the year (including the current pay period).
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution).
*
*   D: Employee’s year-to-date (before the pay period) Canada Pension Plan contribution with the employer
*
*   D2: Employee’s year-to-date (before the pay period) second additional Canada Pension Plan contribution with the employer
*
*   PEytd: Employee's year-to-date (before the pay period) pensionable earnings
*
*   D1: Employee’s year-to-date (before the pay period) employment insurance premium with the employer
*
*   EIytd: Employee's year-to-date (before the pay period) Insurable earnings
*
*   B1: Gross bonuses, retroactive pay increases, vacation pay when vacation is not taken, accumulated overtime payments or other non-periodic payments year-to-date (before the pay period)
*
*  Note: For overtime earned and paid in the same pay period, the payment is included with the I factor. Also, when the employee gets vacation pay and takes vacation, the income is included in the I factor. If you want to make deductions such as RRSP contributions from the bonus payment, see the instructions in Option 1 for using factors F3 and F4.
*
*   M: Accumulated federal and provincial or territorial tax deductions (if any) to the end of the last pay period
*
*   Do not include any year‑to‑date extra tax deductions requested by the employee, factor L. Tax
*   already deducted on non-periodic payments such as bonuses, is included
*   in factor M1
*
*   M1: Year-to-date tax deducted on all payments included in B1
*
*   Accumulated federal and provincial (or territorial) tax deductions on non-periodic payments
*   such as bonuses, if any, to the last pay period. Do not include any
*   year‑to‑date extra tax deductions for the year requested by the
*   employee, factor L or any tax included in factor M. The T factor (tax deduction for the pay
*   period) will not include the tax on the non-periodic payment. The tax to be deducted on a
*   current non‑periodic payment is kept in another field
*   TB.
*
*   F: Payroll deductions for the pay period for employee contributions to a registered pension plan (RPP) for current and past services, a registered retirement savings plan (RRSP), to a pooled registered pension plan (PRPP), or a retirement compensation arrangement (RCA). For tax deduction purposes, employers can deduct amounts contributed to an RPP, RRSP, PRPP, or RCA by or on behalf of an employee to determine the employee's taxable income
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
*   F4: Employee registered pension plan or registered retirement savings plan contributions deducted from the year-to-date non-periodic payments. You can also use this field or design another to apply other tax-deductible amounts to the non-periodic payment, such as union dues
*
*   K3P: Other provincial or territorial non-refundable tax credits (such as medical expenses and charitable donations) authorized by a tax services office or tax centre
*
*   lives_outside_city_limits: outside Canada and in Canada beyond the limits of any province or territory.
*
*   number_of_disabled_dependants: Number of disabled dependants
*
*   number_of_minor_dependents: Number of dependents under the age of 19
*/
#[allow(non_snake_case)]
pub struct TaxPayerVariables {
    pub I: f64,
    pub TCP: f64,
    pub P: i64,
    pub PR: i64,
    pub PM: i64,
    pub D: f64,
    pub D2: f64,
    pub PIytd: f64,
    pub D1: f64,
    pub EIytd: f64,
    pub B1: f64,
    pub M: f64,
    pub M1: f64,
    pub F: Option<f64>,
    pub L: Option<f64>,
    pub F1: Option<f64>,
    pub HD: Option<f64>,
    pub U1: Option<f64>,
    pub F2: Option<f64>,
    pub F4: Option<f64>,
    pub K3P: Option<f64>,
    pub lives_outside_city_limits: bool,
    pub number_of_disabled_dependants: Option<i64>,
    pub number_of_minor_dependents: Option<i64>,
}

impl TaxPayerVariables {
    #[allow(non_snake_case)]
    pub fn new(
        I: f64,
        TCP: f64,
        P: i64,
        PR: i64,
        PM: i64,
        D: f64,
        D2: f64,
        PIytd: f64,
        D1: f64,
        EIytd: f64,
        B1: f64,
        M: f64,
        M1: f64,
        F: Option<f64>,
        L: Option<f64>,
        F1: Option<f64>,
        HD: Option<f64>,
        U1: Option<f64>,
        F2: Option<f64>,
        F4: Option<f64>,
        K3P: Option<f64>,
        lives_outside_city_limits: bool,
        number_of_disabled_dependants: Option<i64>,
        number_of_minor_dependents: Option<i64>,
    ) -> Self {
        TaxPayerVariables {
            I,
            TCP,
            P,
            PR,
            PM,
            D,
            D2,
            PIytd,
            D1,
            EIytd,
            B1,
            M,
            M1,
            F,
            L,
            F1,
            HD,
            U1,
            F2,
            F4,
            K3P,
            lives_outside_city_limits,
            number_of_disabled_dependants,
            number_of_minor_dependents,
        }
    }

    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub fn __test__(tax_constants: &TaxConstants, I: Option<f64>) -> Result<Self, Box<dyn Error>> {
        let TCP: f64 = tax_constants.prov.ORA.get_basic_amount_value().unwrap();
        let i = match I {
            Some(x) => x,
            None => 50000.0,
        };
        Ok(TaxPayerVariables::new(
            i, TCP, 52, 52, 12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None, None, None,
            None, None, None, None, false, None, None,
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

        let result = TaxPayerVariables::__test__(&tax_constants, None);
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

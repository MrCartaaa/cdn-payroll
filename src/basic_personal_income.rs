//! # Basic Personal Amount Calculation
//! The Basic Personal Amount (BPA) is a non-refundable tax credit that all individuals can claim in Canada. It provides a full reduction from federal income tax for individuals with taxable income below the BPA and a partial reduction for those with taxable income above it. 
//! It's important to note that the BPA is adjusted annually due to inflation and government policy.

use crate::utils;
use crate::year::v2025;

/** Calculate Federal Basic Personal Amount.
*
*
* Given:
*
*   A: Annual Taxable Income
*
*   HD: Annual deduction for living in a prescribed zone, as shown on Form TD1
*
* Where:
*
*   NI: Net Income
*
*   NI = A + HD
*/
#[allow(non_snake_case)]
pub fn BPAF(A: f64, HD: f64) -> Result<f64, f64> {
    let mut BPAF: f64 = 0.0;
    let NI = A+HD;

    if NI <= v2025::INCOME_THRESHOLD_4 {
        BPAF = v2025::MINIMUM_BASIC_AMT;
    } else
    if v2025::INCOME_THRESHOLD_4 < NI && NI < v2025::INCOME_THRESHOLD_5 {
        BPAF = v2025::MINIMUM_BASIC_AMT - (NI*-v2025::INCOME_THRESHOLD_4) * (1591.0 / 75532.0);
    } else
    if NI > v2025::INCOME_THRESHOLD_5 {
        BPAF = v2025::MAXIMUM_BASIC_AMT;
    }

    if BPAF == 0.0 {
        return Err(0.0)
    }

    Ok(utils::round(BPAF))
}

/** Calculate Non-Commissionable Income Tax.
*
*
* Given:
*
*   P: number of pay periods in the year.
*
*   I: Gross remuneration for the pay period.
*
*   This includes overtime earned and paid in the same pay period, pension income, qualified pension income, and taxable benefits, but does not include bonuses, retroactive pay increases, or other non-periodic payments
*
*   F: Payroll deductions for the pay period for employee contributions to a registered pension plan (RPP) for current and past services, a registered retirement savings plan (RRSP), to a pooled registered pension plan (PRPP), or a retirement compensation arrangement (RCA).
*
*   For tax deduction purposes, employers can deduct amounts contributed to an RPP, RRSP, PRPP, or RCA by or on behalf of an employee to determine the employee's taxable income
*
*   F2: Alimony or maintenance payments required by a legal document dated before May 1, 1997, to be payroll-deducted authorized by a tax services office or tax centre
*
*   F5A: Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income
*
*   U1: Union dues for the pay period paid to a trade union, an association of public servants, or dues required under the law of a province to a parity or advisory committee or similar body
*
*   HD: Annual deduction for living in a prescribed zone, as shown on Form TD1
*
*   F1: Annual deductions such as child care expenses and support payments requested by an employee or pensioner and authorized by a tax services office or tax centre
*
*   T: Estimated federal and provincial or territorial tax deductions for the pay period
*
*   L: Additional tax deductions for the pay period requested by the employee or pensioner as shown on Form TD1
*/
#[allow(non_snake_case)]
pub fn A(P: i64, I: f64, F: f64, F2: f64, F5A: f64, U1: f64, HD: f64, F1: f64, mut T: f64, L: f64) -> (f64, f64) {
    let a: f64;
    a = P as f64 * (I - F - F2 -F5A -U1) - HD - F1;
    if a.is_sign_negative() {
        T = L
    }
    (utils::round(a), T)
}

/** Calculate Non-Commissionable Income Tax
*
*  Using Cumulative Average Calculation
*
* Given:
*
*  S1: Annualizing factor
*
*  I: Gross pay for the pay period. This includes overtime earned and paid in the same pay period, pension income, qualified pension income, and taxable benefits, plus IYTD, but does not include amounts in factor B.
*
*  F: Payroll deductions for the pay period for employee contributions to a registered pension plan for current and past services, a registered retirement savings plan (RRSP), or a retirement compensation arrangement plus FYTD.
*
*  F1: Federal non-refundable personal tax credit (the lowest federal tax rate is used to calculate this credit)
*
*  F2: Alimony or maintenance payments required by a legal document dated before May 1, 1997, to be deducted at source from the employee’s salary for the pay period plus F2YTD. The legal document could be a garnishment or a similar order of a court or competent tribunal.
*
*  F4: Employee registered pension plan or registered retirement savings plan contributions deducted from the year‑to‑date non-periodic payments. You can also use this field or design another to apply other tax-deductible amounts to the non-periodic payment, such as union dues.
*
*  F5A: Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income plus F5AYTD.
*
*  F5B: Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the non-periodic income plus F5BYTD.
*
*  U1: Union dues for the pay period, plus U1YTD.
*
*  B1: Year‑to‑date (before this pay period) non-periodic payments such as bonuses, retroactive pay increases, vacation pay when vacation is not taken, and accumulated overtime. Since the tax on a current non-periodic payment is calculated separately, do not include the current non-periodic payment in calculating A.
*
*  Note: For overtime earned and paid in the same pay period, the payment is included with the I factor. Also, when the employee gets vacation pay and takes vacation, the income is included in the I factor. If you want to make deductions such as RRSP contributions from the bonus payment, see the instructions in Option 1 for using factors F3 and F4.
*
*  HD: Annual deduction for living in a prescribed zone, as shown on Form TD1
*/
#[allow(non_snake_case)]
pub fn A_grad(S1: f64, I: f64, F: f64, F1: f64, F2: f64, F4: f64, F5A: f64, F5B: f64, U1: f64, B1: f64, HD: f64) -> f64 {
    let a: f64 = (S1 * (I - F - F2 - F5A - U1)) + (B1 - F4 - F5B) - HD - F1;
    if a.is_sign_negative() {
        return 0.0;
    }
    a
}

/** Annualizing factor
*
* Given:
*
*   total_pay_periods: the number of total pay periods (or the employee’s pay periods if the employees worked less than the total pay periods)
*   current_pay_period: current pay period
*/
#[allow(non_snake_case)]
pub fn S1(total_pay_periods: i64, current_pay_period: i64) -> f64 {
    (total_pay_periods / current_pay_period) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_BPAF_minimum_amt() {
        let result = BPAF(10000.0, 0.0);
        assert_eq!(result.unwrap(), v2025::MINIMUM_BASIC_AMT);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BPAF_maximum_amt() {
        let result = BPAF(253414.01, 0.0);
        assert_eq!(result.unwrap(), v2025::MAXIMUM_BASIC_AMT);
    }

}


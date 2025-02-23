//! Formulas to calculate the estimated federal and provincial or territorial tax deductions (T) for the pay period

use crate::utils;

/** Estimated federal and provincial or territorial tax deductions for the pay period
*       (Non-Commissionable earnings)
*
*
* Given:
*
*   T1: Annual federal tax deduction
*
*   T2: Annual provincial or territorial tax deduction (except Quebec)
*
*   P: The number of pay periods in the year
*
*   L: Additional tax deductions for the pay period requested by the employee or pensioner as shown on Form TD1
*/
#[allow(non_snake_case)]
pub fn T(T1: f64, T2: f64, P: i64, L: f64) -> f64 {
    utils::round(((T1 + T2) / P as f64) + L)
}

/** Estimated Federal and Provincial or Territorial Tax Deductions for the Pay Period
*
*   Uses Cumulative Average Calculation
*
* Given:
*
*   T1_grad: Annual federal tax deduction (Uses cumalitve average calculation)
*
*   T2: Annual provincial or territorial tax deduction (except Quebec)
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
*   S1: Annualizing factor
*
*   M: Accumulated federal and provincial or territorial tax deductions (if any) to the end of the last pay period
*
*   Do not include any year‑to‑date extra tax deductions requested by the employee, factor L. Tax
*   already deducted on non-periodic payments such as bonuses, is included
*   in factor M1
*
*   L: Additional tax deductions for the pay period requested by the employee or pensioner as shown on Form TD1
*/
#[allow(non_snake_case)]
pub fn T_grad(T1_grad: f64, T2: f64, M1: f64, S1: f64, M: f64, L: f64) -> f64 {
    let t: f64;

    t = ((T1_grad + T2 - M1) / S1) - M;
    if t.is_sign_negative() {
        return L;
    }

    utils::round(t + L)
}


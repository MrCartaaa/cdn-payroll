//! Formulas to calculate the estimated federal and provincial or territorial tax deductions (T) for the pay period


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
    ((T1 + T2) / P as f64) + L
}


//! Annual Basic Provincial or Territorial Tax

use crate::utils;
use crate::year::v2025;

/** Annual basic provincial or territorial tax
*
*
* Given:
*
*   V: Provincial or territorial tax rate for the year
*
*   A: Annual taxable income
*
*   KP: Provincial or territorial constant
*
*   K2P: Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year
*         Note: If an employee has already contributed the maximum CPP and EI, for the year with the employer, use the maximum base CPP contribution and the maximum EI premium to calculate the credit for the rest of the year. If, during the pay period in which the employee reaches the maximum, the CPP and  EI, when annualized, is less than the annual maximum, use the maximum base CPP contribution and the maximum EI premium in that pay period
*
*   K3P: Other provincial or territorial non-refundable tax credits
*
*   4P: Territorial non-refundable tax credit calculated using the provincial or territorial Canada employment amount
*/
#[allow(non_snake_case)]
pub fn T4(V: f64, A: f64, KP: f64, K1P: f64, K2P: f64, K3P: f64, K4P: f64) -> f64 {
    let t4: f64 = (V * A) - KP - K1P - K2P - K3P - K4P;
    if t4 < 0.0 {
        return 0.0;
    }
    utils::round(t4)
}

/** Annual provincial or territorial tax deduction (except Quebec)
*
*
* Given:
*
*   T4: Annual basic provincial or territorial tax
*
*   V1: Provincial surtax calculated on the basic provincial tax (only applies to Ontario)
*
*   V2: Additional tax calculated on taxable income (only applies to the Ontario Health Premium)
*
*   S: Provincial tax reduction (only applies to Ontario and British Columbia)
*
*   P: The number of pay periods in the year
*
*  LCP: Provincial or territorial labour-sponsored funds tax credit
*/
#[allow(non_snake_case)]
pub fn T2(T4: f64, V1: f64, V2: f64, S: f64, P: i64, LCP: f64) -> f64 {
    let t2: f64 = T4 + V1 + V2 - S - (P as f64 * LCP);
    if t2 < 0.0 {
        return 0.0;
    }
    utils::round(t2)
}

/** Provincial or territorial non-refundable personal tax credit
*    (the lowest tax rate of the province or territory is used to calculate this credit) 
*
*
* Given:
*
*   lowest_provincial_tax_rate: Lowest provincial tax rate
*
*   TCP: "Total claim amount," reported on the provincial or territorial Form TD1.
*/
#[allow(non_snake_case)]
pub fn K1P(lowest_provincial_tax_rate: f64, TCP: f64) -> f64 {
    utils::round(lowest_provincial_tax_rate * TCP)
}

/** Provincial or territorial base Canada Pension Plan contributions and employment insurance premiums tax credits for the year (the lowest provincial or territorial tax rate is used to calculate this credit).
*      If an employee reaches the maximum CPP or EI for the year with an employer, the instructions in the note for the K2 factor also apply to the K2P factor. For employees paid by commission, use the federal K2 formula for commissions and replace the lowest federal rate in the K2 formula with the lowest provincial or territorial tax rate
*
*  Given:
*
*   lowest_provincial_tax_rate:
*
*   P: The number of pay periods in the year
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution).
*
*   C: Canada (or Quebec) Pension Plan contributions for the pay period
*
*   EI: Employment insurance premiums for the pay period
*/
#[allow(non_snake_case)]
pub fn K2P(lowest_provincial_tax_rate: f64, P: i64, PM: i64, C: f64, EI: f64) -> f64 {
    let mut k2p: f64;

    let mut cpp: f64 = P as f64 * C * (0.0495/0.0595);
    if cpp > v2025::CPP_MAX_CONTRIBUTIONS {
        cpp = v2025::CPP_MAX_CONTRIBUTIONS;
    }
    k2p = lowest_provincial_tax_rate * (cpp * (PM/12) as f64);

    let mut ei: f64 = P as f64 * EI;
    if ei > v2025::EI_MAX_CONTRIBUTIONS {
        ei = v2025::EI_MAX_CONTRIBUTIONS;
    }
    k2p += lowest_provincial_tax_rate * ei;

    utils::round(k2p)
}


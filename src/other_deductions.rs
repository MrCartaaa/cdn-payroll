//! Canadian Pension Plan and Employee Insurance Deductions

use crate::utils;
use crate::year::v2025;

//
// Canada Pension Plan Calculations:
//


/** Canada (or Quebec) Pension Plan contributions for the pay period (Non-Commissionable Earnings)
*
* Given:
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution).
*
*   D: Employee’s year-to-date (before the pay period) Canada Pension Plan contribution with the employer
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
*   P: The number of pay periods in the year
*/
#[allow(non_snake_case)]
pub fn C(PM: i64, D: f64, PI: f64, P: i64) -> f64 {
    let c1: f64 = 4034.1 * (PM/12) as f64 - D;
    let c2: f64 = 0.0595 * (PI - (3500.0 / P as f64));
    if c1 < c2 {
        return utils::round(c2);
    } else {
        return utils::round(c1);
    }
}

/** Second additional Canada (or Quebec) Pension Plan contributions for the pay period
*
* Given:
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution).
*
*   D2: Employee’s year-to-date (before the pay period) second additional Canada Pension Plan contribution with the employer
*
*   PI_YTD: Year-to-date pensionable earnings, or the year-to-date gross income plus any taxable benefits, including bonuses and retroactive pay increases where applicable
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
*   W: The greater of year-to-date (before the pay period) pensionable earnings (PIYTD or GYTD) and employee’s Year’s Maximum Pensionable Earnings (YMPE).
*/
#[allow(non_snake_case)]
pub fn C2(PM: i64, D2: f64, PI_YTD: f64, PI: f64, W: f64) -> f64 {
    let c21: f64 = 396.0 * (PM/12) as f64 - D2;
    let c22: f64 = (PI_YTD + PI - W) * 0.04;
    let mut c2: f64;
    if c21 < c22 {
        c2 = c21;
    } else {
        c2 = c22;
    }
    if c2.is_sign_negative() {
        c2 = 0.0;
    }

    utils::round(c2)
}

/** Year-to-Date Pensionable Earnings (PI_YTD) (or employee's Year's Maximum Pensionable Earnings (YMPE))
*
* Given:
*
*   PI_YTD: Year-to-date pensionable earnings, or the year-to-date gross income plus any taxable benefits, including bonuses and retroactive pay increases where applicable
*
*   YMPE: Year's Maximum Pensionable Earnings
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution).
*/
#[allow(non_snake_case)]
pub fn W(PI_YTD: f64, YMPE: f64, PM: i64) -> f64 {
    let w1: f64 = YMPE * (PM/12) as f64;

    if w1 > PI_YTD {
        return utils::round(w1);
    }
    PI_YTD
}


//
// Employee Insurance Calculations:
//


/** Employment insurance premiums for the pay period
*
* Given:
*
*   D1: Employee’s year-to-date (before the pay period) employment insurance premium with the employer
*
*   IE: Insurable earnings for the pay period, including insurable taxable benefits, bonuses, and retroactive pay increases
*/
#[allow(non_snake_case)]
pub fn EI(D1: f64, IE: f64) -> f64 {
    let ei1: f64 = v2025::EI_MAX_CONTRIBUTIONS - D1;
    let ei2: f64 = 0.0164 * IE;
    if ei1 < ei2 {
        return utils::round(ei1);
    } else {
        return utils::round(ei2);
    }
}


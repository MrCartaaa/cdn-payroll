//! # Income Tax Calculations.
//!

use crate::utils;
use crate::year::v2025;

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

/** Calculate Annual Deductions.
*
* If F1 amount is implemented after the first pay period of the year, it must be calculated.
*
*
* Given:
*
*   P: number of pay periods in the year.
*
*   F1: total annual deductions
*
*   PR: number of pay periods left in the year (including the current pay period)
*/
#[allow(non_snake_case)]
pub fn F1(P: i64, PR: i64, F1: f64) -> f64 {
    utils::round((P as f64 * F1) / PR as f64)
}

/** Deductions for Canada Pension Plan additional contributions for the pay period.
*
*    NOTE, A separate formula is used for non-commissionable earnings.
*
*
*
* Given:
*
*    C: Canada (or Quebec) Pension Plan contributions for the pay period
*
*   C2: Second additional Canada (or Quebec) Pension Plan contributions for the pay period
*/
#[allow(non_snake_case)]
pub fn F5(C: f64, C2: f64) -> f64 {
    if C == 0.0 && C2 == 0.0 {
        return 0.0
    }
    utils::round(C * (0.100/0.0595) + C2)
}



/** Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income
*
*
* Given:
*
*  F5: Deductions for Canada Pension Plan additional contributions for the pay period
*
* Use F5Q inplace of F5 for Quebec: Deductions for Quebec Pension Plan additional contributions for the pay period
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
*   B: Gross bonus, retroactive pay increase, vacation pay when vacation is not taken, accumulated overtime payment or other non-periodic payment
*/
#[allow(non_snake_case)]
pub fn F5A(F5: f64, PI: f64, B: f64) -> f64 {
    utils::round(F5 * ((PI - B) / PI))
}

/** Annual basic federal tax
*
*
* Given:
*
*   R: Federal tax rate that applies to the annual taxable income A
*
*   A: Annual taxable income
*
*   K: Federal constant. The constant is the tax overcharged when applying the 20.5%, 26%, 29%, and 33% rates to the annual taxable income A
*
*   K1: Federal non-refundable personal tax credit (the lowest federal tax rate is used to calculate this credit)
*
*   K2: Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year (the lowest federal tax rate is used to calculate this credit).
*
*      Replace K2 with K2R where: employees that are transferred from Quebec to a location outside Quebec
*
*   K3: Other federal non-refundable tax credits (such as medical expenses and charitable donations) authorized by a tax services office or tax centre
*
*   K4: Federal non-refundable tax credit calculated using the Canada employment amount (the lowest federal tax rate is used to calculate this credit)
*/
#[allow(non_snake_case)]
pub fn T3(R: f64, A: f64, K: f64, K1: f64, K2: f64, K3: f64, K4: f64) -> f64 {
    let result: f64 = (R * A) - K - K1 - K2 - K3 - K4;
    if result.is_sign_negative() {
        return 0.0;
    }
    utils::round(result)
}

/** Federal non-refundable personal tax credit (the lowest federal tax rate is used to calculate this credit)
*
*
* Given:
*
*   TC: “Total claim amount,” reported on federal Form TD1.
*/
#[allow(non_snake_case)]
pub fn K1(TC: f64) -> f64 {
    0.15 * TC
}

/** Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year
*
*
* Given:
*
*   P: The number of pay periods in the year
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted
*
*   C: Canada (or Quebec) Pension Plan contributions for the pay period
*
*   EI: Employment insurance premiums for the pay period
*/
#[allow(non_snake_case)]
pub fn K2(P: i64, PM: i64, C: f64, mut EI: f64) -> f64 {

    if EI > v2025::EI_MAX_CONTRIBUTIONS {
        EI = v2025::EI_MAX_CONTRIBUTIONS;
    }

    let mut result = 0.15 * (P as f64 * C * (0.0495 / 0.0595));
    //TODO: check if the `result` is anywhere near CPP_MAX_CONTRIBUTIONS; not sure if I've writen
    //this correctly
    if result > v2025::CPP_MAX_CONTRIBUTIONS {
        result = v2025::CPP_MAX_CONTRIBUTIONS;
    }

    result = (result * (PM/12) as f64) + (0.15 * (P as f64 * EI));

    utils::round(result)
}

/** Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year
*
*       Calculated using the year-to-date method
*
*
* Given:
*
*   PM: The total number of months during which CPP and/or QPP contributions are required to be deducted
*
*   PR: The number of pay periods left in the year (including the current pay period)
*
*   C: Canada (or Quebec) Pension Plan contributions for the pay period
*
*   D: Employee’s year-to-date (before the pay period) Canada Pension Plan contribution with the employer
*
*   D1: Employee’s year-to-date (before the pay period) employment insurance premium with the employer
*
*   EI: Employment insurance premiums for the pay period
*/
#[allow(non_snake_case)]
pub fn K2_YTD(PM: i64, PR: i64, C: f64, D: f64, D1: f64, EI: f64) -> f64 {
    let mut result: f64 = 0.15;
    let cpp_ftc1: f64 = v2025::CPP_MAX_CONTRIBUTIONS * (PM/12) as f64;
    let cpp_ftc2: f64 = (D * (0.0495/0.0595)) + (PR as f64 * C * (0.0495/0.0595));
    if cpp_ftc1 > cpp_ftc2 {
        result *= cpp_ftc2
    } else {
        result *= cpp_ftc1
    }

    let ei_ftc: f64;
    let y: f64 = D1 + (PR as f64 * EI);
    if y > v2025::EI_MAX_CONTRIBUTIONS {
        ei_ftc = v2025::EI_MAX_CONTRIBUTIONS;
    } else {
        ei_ftc = y;
    }

    result += 0.15 * ei_ftc;
    utils::round(result)
}

/** Other federal non-refundable tax credits
*
*
* Given:
*
*   P: The number of pay periods in the year
*
*   PR: The number of pay periods left in the year (including the current pay period)
*
*   K3: Other federal non-refundable tax credits
*/
#[allow(non_snake_case)]
pub fn K3(P: i64, PR: i64, K3: f64) -> f64 {
    (P as f64 * K3) / PR as f64
}

/** Federal non-refundable tax credit calculated using the Canada employment amount (the lowest federal tax rate is used to calculate this credit)
*
*
* Given:
*
*   A: Annual taxable income
*
*   CEA: Canada Employment Amount, a non-refundable tax credit used in the calculation for K4 and K4P
*/
#[allow(non_snake_case)]
pub fn K4(A: f64, CEA: f64) -> f64 {
    let k41: f64 = 0.15 * A;
    let k42: f64 = 0.15 * CEA;
    if k41 > k42 {
        return utils::round(k42);
    } else {
        return utils::round(k41);
    }
}

/** Annual federal tax deduction
*
*
* Given:
*
*   T3: Annual basic federal tax
*
*   P: The number of pay periods in the year
*
*   LCF: Federal labour-sponsored funds tax credit
*/
#[allow(non_snake_case)]
pub fn T1(T3: f64, P: i64, LCF: f64) -> f64 {
    let t1: f64;
    t1 = T3 - (P as f64 * LCF);
    if t1.is_sign_negative() {
        return 0.0;
    }
    utils::round(t1)
}

/** Federal labour-sponsored funds tax credit
*
*
* Given:
*
*   acquisition_pay_loss: Fifteen percent of the amount deducted or withheld for the pay period for the acquisition, by the employee, of approved shares of the capital stock of a prescribed labour-sponsored venture capital corporation
*/
#[allow(non_snake_case)]
pub fn LCF(acquisition_pay_loss: f64) -> f64 {
    let lcf: f64 = 0.15 * acquisition_pay_loss;
    if 750.0 > lcf {
        return utils::round(acquisition_pay_loss);
    } else {
        return 750.0;
    }
}


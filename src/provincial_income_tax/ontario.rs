//! Ontario Provincial Income Tax

use crate::utils;
use crate::context::Context;

/** # Provincial surtax calculated on the basic provincial tax (only applies to Ontario)
*
*
* ## Arguements:
*
*   T4: Annual basic provincial or territorial tax
*
* ## Examples:
*
* ```
* use cdn_payroll::context::{Context, Version};
* use cdn_payroll::provincial_income_tax::ontario::V1;
*
* let ctx = Context::new(Version::V2025_1).unwrap();
* let t4 = 5400.0;
* let v1 = V1(ctx, &t4);
* assert_eq!(v1, Ok(0.0));
* ```
*/
#[allow(non_snake_case)]
pub fn V1(ctx: Context, T4: &f64) -> Result<f64, &'static str> {
    let ctx_ora_on = &ctx.ORA.ON;

    let t4atv1 = &ctx_ora_on.T4atV1.as_ref().ok_or_else(|| "unable to locate V1 at T4[x].")?;
    let v1 = &ctx_ora_on.V1Rate.as_ref().ok_or_else(|| "unable to locate V1 Rates for T4.")?;

    let t4atv1_1 = t4atv1.get(1).ok_or_else(|| "unable to locate V1 at T4[x] (level 1).")?;
    let v1_1 = v1.get(1).ok_or_else(|| "unable to locate V1 Rates for T4 (level 1).")?;

    let t4atv1_2 = t4atv1.get(2).ok_or_else(|| "unable to locate V1 at T4[x] (level 2).")?;
    let v1_2 = v1.get(2).ok_or_else(|| "unable to locate V1 Rate (level 2) for T4.")?;

    if T4 <= t4atv1_1 {
        return Ok(0.0);
    } else

    if T4 > t4atv1_1 && T4 < t4atv1_2 {
        return Ok(utils::round(v1_1 * (T4 - t4atv1_1)));
    } else
    // if T4 > t4atv1_2
    {
        return Ok(utils::round(v1_1 * (T4 - t4atv1_1) + (v1_2 * (T4 - t4atv1_2))));
    }
}

/** Additional tax calculated on taxable income (only applies to the Ontario Health Premium)
*
*
* Given:
*
*   A: Annual taxable income
*/
#[allow(non_snake_case)]
pub fn V2(A: f64) -> f64 {
    let v2: f64;
    if A < 20000.0 {
        return 0.0;
    } else

    if A > 20000.0 && A < 36000.0 {
        v2 = 0.06 * (A - 20000.0);
        if v2 < 300.0 {
                return utils::round(v2);
            } else {
                return 300.0;
            }
    } else

    if A > 36000.0 && A < 48000.0 {
        v2 = 300.0 + (0.06 * (A - 36000.0));
        if v2 < 450.0 {
            return utils::round(v2);
        } else {
            return 450.0;
        }
    } else

    if A > 48000.0 && A < 72000.0 {
        v2 = 600.0 + (0.25 * (A - 72000.0));
        if v2 < 750.0 {
            return utils::round(v2);
        } else {
            return 750.0;
        }
    } else

    if A > 72000.0 && A < 200000.0 {
        v2 = 600.0 + (0.25 * (A - 72000.0));
        if v2 < 900.0 {
            return utils::round(v2);
        } else {
            return 900.0;
        }
    } else
    // if A > 200000.0
    {
        v2 = 750.0 + (0.25 * (A - 200000.0));
        if v2 < 900.0 {
            return utils::round(v2);
        }
        else {
            return 900.0;
        }
    }
}

/** Provincial tax reduction (only applies to Ontario and British Columbia)
*
*
* Given:
*
*   T4: Annual basic provincial or territorial tax
*
*   V1: Provincial surtax calculated on the basic provincial tax (only applies to Ontario)
*
*   Y: Additional provincial tax reduction amount based on the number of eligible dependents used in the calculation of Factor S (only applies to Ontario)
*/
#[allow(non_snake_case)]
pub fn S(T4: f64, V1: f64, Y: i64) -> f64 {
    let s1: f64 = T4 + V1;
    let s2: f64 = (2.0 * 294.0 + Y as f64) - (T4 + V1);
    if s1 < 0.0 && s2 < 0.0 {
        return 0.0;
    }

    if s1 < s2 {
        if s1 < 0.0 {
            return 0.0;
        }
        return s1;
    } else {
        if s2 < 0.0 {
            return 0.0;
        }
        return utils::round(s2);
    }
}

/** Additional provincial tax reduction amount based on the number of eligible dependents used in the calculation of Factor S (only applies to Ontario)
*
*
* Given:
*
*   number_of_disabled_dependants: Number of disabled dependants
*
*   number_of_minor_dependents: Number of dependents under the age of 19
*/
#[allow(non_snake_case)]
pub fn Y(number_of_disabled_dependants: i64, number_if_minor_dependents: i64) -> f64 {
    544.0 * number_of_disabled_dependants as f64 + 544.0 * number_if_minor_dependents as f64
}


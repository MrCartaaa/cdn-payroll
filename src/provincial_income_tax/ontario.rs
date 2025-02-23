//! Ontario Provincial Income Tax

use crate::utils;

/** Provincial surtax calculated on the basic provincial tax (only applies to Ontario)
*
*
* Given:
*
*   T4: Annual basic provincial or territorial tax
*/
#[allow(non_snake_case)]
pub fn V1(T4: f64) -> f64 {
    // TODO: these fixed numbers have to be extracted from the csv file 'thrrtsmnts-01-25e.csv'
    if T4 <= 5710.0 {
        return 0.0;
    } else

    if T4 > 5710.0 && T4 < 7307.0 {
        return 0.2 * (T4 - 5710.0);
    } else
    // if T4 > 7307.0
    {
        return utils::round(0.2 * (T4 - 5710.0) + 0.36 * (T4 - 7307.0));
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


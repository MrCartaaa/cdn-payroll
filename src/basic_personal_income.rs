//! # Basic Personal Amount Calculation
//! The Basic Personal Amount (BPA) is a non-refundable tax credit that all individuals can claim in Canada. It provides a full reduction from federal income tax for individuals with taxable income below the BPA and a partial reduction for those with taxable income above it.
//! It's important to note that the BPA is adjusted annually due to inflation and government policy.

use crate::context;
use crate::context::Context;
use crate::utils;

/** ## Calculate Federal Basic Personal Amount.
*
* ### Arguements:
*
*   ctx: Context
*
*   [A](./fn.A.html): Annual Taxable Income
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn BPAF(ctx: Context, A: &f64) -> f64 {
    let BPAF: f64;
    let hd = match ctx.payer_vars.HD {
        Some(x) => x,
        None => 0.0,
    };
    let NI = A + hd;

    let income_threshold_4 = ctx.tax_consts.fed.RITC.A.get(3).unwrap();
    let income_threshold_5 = ctx.tax_consts.fed.RITC.A.get(4).unwrap();

    if &NI <= income_threshold_4 {
        BPAF = context::MINIMUM_BASIC_AMT;
    } else if income_threshold_4 < &NI && &NI < income_threshold_5 {
        //TODO: The hard coded 1591 / 75k has to be replaced with tax_consts (but I am unable to
        //find it; might need to create a new table or add it to an existing one.)
        BPAF = context::MINIMUM_BASIC_AMT - (&NI * -income_threshold_5) * (1591.0 / 75532.0);
    } else
    // if NI > income_threshold_5
    {
        BPAF = context::MAXIMUM_BASIC_AMT;
    }

    utils::round(BPAF)
}

/** ## Calculate Non-Commissionable Income Tax.
*
*
* ### Arguements:
*
*   ctx: Context
*
*   [F5A](../federal_income_tax/fn.F5A.html): Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income
*
*   T - [[T](../income_tax/fn.T.html) or [T_grad](../income_tax/fn.T_grad.html)] : Estimated federal and provincial or territorial tax deductions for the pay period
*
* ### Examples:
* TODO: Add examples...
*
*/
#[allow(non_snake_case)]
pub fn A(ctx: Context, F5A: f64, mut T: f64) -> (f64, f64) {
    let a: f64;
    let f = match ctx.payer_vars.F {
        Some(x) => x,
        None => 0.0,
    };
    let u1 = match ctx.payer_vars.U1 {
        Some(x) => x,
        None => 0.0,
    };
    let hd = match ctx.payer_vars.HD {
        Some(x) => x,
        None => 0.0,
    };
    let f1 = match ctx.payer_vars.F1 {
        Some(x) => x,
        None => 0.0,
    };
    let f2 = match ctx.payer_vars.F2 {
        Some(x) => x,
        None => 0.0,
    };
    let l = match ctx.payer_vars.L {
        Some(x) => x,
        None => 0.0,
    };
    a = ctx.payer_vars.P as f64 * (ctx.payer_vars.I - f - f2 - F5A - u1) - hd - f1;
    if a.is_sign_negative() {
        T = l
    }
    (utils::round(a), T)
}

/** ## Calculate Non-Commissionable Income Tax
*
*  Using Cumulative Average Calculation
*
* ### Arguements:
*
*   ctx: Context
*
*  [S1](./fn.S1.html): Annualizing factor
*
*  [F5A](../federal_income_tax/fn.F5A.html): Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income plus F5AYTD.
*
*  [F5B](../federal_income_tax/fn.F5B.html): Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the non-periodic income plus F5BYTD.
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn A_grad(ctx: &Context, S1: f64, F5A: f64, F5B: f64) -> f64 {
    let f = match ctx.payer_vars.F {
        Some(x) => x,
        None => 0.0,
    };
    let f1 = match ctx.payer_vars.F1 {
        Some(x) => x,
        None => 0.0,
    };
    let f2 = match ctx.payer_vars.F2 {
        Some(x) => x,
        None => 0.0,
    };
    let f4 = match ctx.payer_vars.F4 {
        Some(x) => x,
        None => 0.0,
    };
    let hd = match ctx.payer_vars.HD {
        Some(x) => x,
        None => 0.0,
    };
    let u1 = match ctx.payer_vars.U1 {
        Some(x) => x,
        None => 0.0,
    };

    let a: f64 =
        (S1 * (ctx.payer_vars.I - f - f2 - F5A - u1)) + (ctx.payer_vars.B1 - f4 - F5B) - hd - f1;
    if a.is_sign_negative() {
        return 0.0;
    }
    a
}

/** ## Annualizing factor
*
* ### Arguements:
*
*   ctx: Context
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn S1(ctx: &Context) -> f64 {
    (ctx.payer_vars.P / (ctx.payer_vars.P - ctx.payer_vars.PR + 1)) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{Context, ProvinceKey, Version};

    #[test]
    #[allow(non_snake_case)]
    fn test_BPAF_minimum_amt() {
        let ctx = Context::new(Version::V2025_1, ProvinceKey::ON, None);
        assert!(ctx.is_ok());
        let result = BPAF(ctx.unwrap(), &10000.0);
        assert_eq!(result, context::MINIMUM_BASIC_AMT);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BPAF_maximum_amt() {
        let ctx = Context::new(Version::V2025_1, ProvinceKey::ON, None);
        assert!(ctx.is_ok());
        let result = BPAF(ctx.unwrap(), &253414.01);
        assert_eq!(result, context::MAXIMUM_BASIC_AMT);
    }
}

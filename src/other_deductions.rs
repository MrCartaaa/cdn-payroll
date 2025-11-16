//! # Canadian Pension Plan and Employee Insurance Deductions

use crate::context::Context;
use crate::utils;

/** ## Canada (or Quebec) Pension Plan contributions for the pay period (Non-Commissionable Earnings)
*
* ### Arguements:
*
*   ctx: Context
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
* ### Examples:
*   // TODO: Create examples
*/
#[allow(non_snake_case)]
pub fn C(ctx: Context, PI: f64) -> f64 {
    let c1: f64 = ctx.tax_consts.CPP.TtlCPP_CRA.MaxEE_ER_TtlCont * (ctx.payer_vars.PM / 12) as f64
        - ctx.payer_vars.D;
    let c2: f64 = ctx.tax_consts.CPP.TtlCPP_CRA.EE_ER_TtlContRate
        * (PI - (ctx.tax_consts.CPP.TtlCPP_CRA.BasicException / ctx.payer_vars.P as f64));
    if c1 < c2 {
        return utils::round(c2);
    } else {
        return utils::round(c1);
    }
}

/** ## Second additional Canada (or Quebec) Pension Plan contributions for the pay period
*
* ### Arguements:
*
*   ctx: Context
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
*   W: The greater of year-to-date (before the pay period) pensionable earnings (PIYTD or GYTD) and employee’s Year’s Maximum Pensionable Earnings (YMPE).
*
* ### Examples:
*   // TODO: create examples
*/
#[allow(non_snake_case)]
pub fn C2(ctx: Context, PI: f64, W: f64) -> f64 {
    let c21: f64 = ctx.tax_consts.CPP.CPPSAddtnlRate.MaxEE_ER_SAddtnlCont
        * (ctx.payer_vars.PM / 12) as f64
        - ctx.payer_vars.D2;
    let c22: f64 =
        (ctx.payer_vars.PIytd + PI - W) * ctx.tax_consts.CPP.CPPSAddtnlRate.EE_ER_SAddtnlContRate;
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

// TODO: This Function takes only context. It can be initialized with context.

/** ## Year-to-Date Pensionable Earnings (PI_YTD) (or employee's Year's Maximum Pensionable Earnings (YMPE))
*
* ### Arguements:
*
*   ctx: Context
*
* ### Examples:
*   //TODO: create examples
*/
#[allow(non_snake_case)]
pub fn W(ctx: Context) -> f64 {
    let w1: f64 = ctx.tax_consts.CPP.TtlCPP_CRA.YMPE * (ctx.payer_vars.PM / 12) as f64;

    if w1 > ctx.payer_vars.PIytd {
        return utils::round(w1);
    }
    ctx.payer_vars.PIytd
}

//
// Employee Insurance Calculations:
//

/** ## Employment insurance premiums for the pay period
*
* ### Arguements:
*
*   ctx: Context
*
*   IE: Insurable earnings for the pay period, including insurable taxable benefits, bonuses, and retroactive pay increases
*
* ### Examples:
*   //TODO: Create examples
*/
#[allow(non_snake_case)]
pub fn EI(ctx: Context, IE: f64) -> f64 {
    let ei1: f64 = ctx.tax_consts.EI.MaxAEEP - ctx.payer_vars.D1;
    let ei2: f64 = ctx.tax_consts.EI.EE_CR * IE;
    if ei1 < ei2 {
        return utils::round(ei1);
    } else {
        return utils::round(ei2);
    }
}

//! # Annual Basic Provincial or Territorial Tax

use crate::context;
use crate::context::Context;
use crate::utils;

/** ## Annual basic provincial or territorial tax
*
*   For cumulative T4 Calculations, use /[x/]_grad in the below list (if not listed, use the normal
*   parameter).
*
*
* ### Arguements:
*
*   V: Provincial or territorial tax rate for the year
*
*   [A](../../basic_personal_income/fn.A.html) \[or [A_grad](../../basic_personal_income/fn.A_grad.html)\]: Annual taxable income.
*
*   KP: Provincial or territorial constant
*
*   [K2P](./fn.K2P.html) \[or [K2P_grad](./fn.K2P.html)\]: Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year.
*
*   Note: If an employee has already contributed the maximum CPP and EI, for the year with the employer, use the maximum base CPP contribution and the maximum EI premium to calculate the credit for the rest of the year. If, during the pay period in which the employee reaches the maximum, the CPP and  EI, when annualized, is less than the annual maximum, use the maximum base CPP contribution and the maximum EI premium in that pay period
*
*   K3P: Other provincial or territorial non-refundable tax credits
*
*   K4P: Territorial non-refundable tax credit calculated using the provincial or territorial Canada employment amount. (currently unimplemented)
*/
#[allow(non_snake_case)]
pub fn T4(
    ctx: &Context,
    V: &f64,
    A: &f64,
    KP: &f64,
    K1P: &f64,
    K2P: &f64,
    K3P: Option<&f64>,
    K4P: Option<&f64>,
) -> Result<f64, &'static str> {
    let k4p = match K4P {
        Some(x) => x,
        None => &0.0,
    };

    let k3p = match K3P {
        Some(x) => x,
        None => &0.0,
    };

    let t4: f64 = (V * A) - KP - K1P - K2P - k3p - k4p;
    if t4 < 0.0 {
        return Ok(0.0);
    }
    Ok(utils::round(t4))
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

/** ## Provincial or territorial base Canada Pension Plan contributions and employment insurance premiums tax credits for the year (the lowest provincial or territorial tax rate is used to calculate this credit).
*
*   If an employee reaches the maximum CPP or EI for the year with an employer, the instructions in the note for the K2 factor also apply to the K2P factor. For employees paid by commission, use the federal K2 formula for commissions and replace the lowest federal rate in the K2 formula with the lowest provincial or territorial tax rate
*
*  ### Arguements:
*
*   ctx: Context
*
*   C: Canada (or Quebec) Pension Plan contributions for the pay period
*
*   EI: Employment insurance premiums for the pay period
*
*   ### Examples:
*       // TODO: Add examples.
*/
#[allow(non_snake_case)]
pub fn K2P(ctx: &Context, C: f64, EI: f64) -> f64 {
    let mut k2p: f64;

    let bccp = ctx.tax_consts.CPP.BaseCPPRate.EE_ER_BaseContRate.to_owned();
    let tccp = ctx.tax_consts.CPP.TtlCPP_CRA.EE_ER_TtlContRate.to_owned();
    let max_cpp_cont = ctx.tax_consts.CPP.BaseCPPRate.MaxEE_ER_TtlCont.to_owned();
    let ltp = ctx.tax_consts.prov.RITC.V[0];
    let max_ei_cont = ctx.tax_consts.EI.MaxAEEP.to_owned();

    let mut cpp: f64 = ctx.payer_vars.P as f64 * C * (bccp / tccp);
    if cpp > max_cpp_cont {
        cpp = max_cpp_cont;
    }
    k2p = ltp * (cpp * (ctx.payer_vars.PM.to_owned() / 12) as f64);

    let mut ei: f64 = ctx.payer_vars.P.to_owned() as f64 * EI;
    if ei > max_ei_cont {
        ei = max_ei_cont;
    }
    k2p += ltp * ei;

    utils::round(k2p)
}

/** ## Provincial or territorial base Canada Pension Plan contributions and employment insurance premiums tax credits for the year (the lowest provincial or territorial tax rate is used to calculate this credit).
*
*   If an employee reaches the maximum CPP or EI for the year with an employer, the instructions in the note for the K2 factor also apply to the K2P factor. For employees paid by commission, use the federal K2 formula for commissions and replace the lowest federal rate in the K2 formula with the lowest provincial or territorial tax rate
*
*   Uses Cumulative Average Calculation
*
*  ### Arguements:
*
*   PE: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period
*
*   S1: Annualizing factor
*
*   EI: Employment insurance premiums for the pay period
*
*   ### Examples:
*       // TODO: Add examples.
*/
#[allow(non_snake_case)]
pub fn K2P_grad(ctx: &Context, PE: f64, S1: f64, EI: f64) -> f64 {
    let mut k2p: f64;

    let ttl_pe = PE + ctx.payer_vars.PIytd;
    let ttl_ei = EI + ctx.payer_vars.EIytd;
    let bccp = ctx.tax_consts.CPP.BaseCPPRate.EE_ER_BaseContRate.to_owned();
    let ltp = ctx.tax_consts.prov.RITC.V[0];
    let max_cpp_cont = ctx.tax_consts.CPP.BaseCPPRate.MaxEE_ER_TtlCont.to_owned();
    let max_ei_cont = ctx.tax_consts.EI.MaxAEEP.to_owned();

    let mut cpp: f64 = (S1 * ttl_pe) + ctx.payer_vars.B1 - ctx.tax_consts.CPP.TtlCPP_CRA.BasicException;
    if cpp.is_sign_negative() {
        cpp = 0.0;
    }
    if cpp > max_cpp_cont {
        cpp = max_cpp_cont;
    }

    k2p = ltp * bccp * cpp;

    let mut ei: f64 = (S1 * ttl_ei) + ctx.payer_vars.B1;
    if ei > max_ei_cont {
        ei = max_ei_cont;
    }
    k2p += ltp * ctx.tax_consts.EI.EE_CR * ei;

    utils::round(k2p)
}

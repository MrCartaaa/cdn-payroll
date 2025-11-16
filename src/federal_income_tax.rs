//! # Federal Income Tax Calculations.
//!

use crate::context::Context;
use crate::utils;

/** ## Calculate Annual Deductions.
*
* If F1 amount is implemented after the first pay period of the year, it must be calculated.
*
* ### Arguments:
*
*   ctx: Context
*
*   [F1](fn.F1.html): total annual deductions
*
* ### Examples:
*   //TODO: create examples...
*/
#[allow(non_snake_case)]
pub fn F1(ctx: Context, F1: f64) -> f64 {
    utils::round((ctx.payer_vars.P as f64 * F1) / ctx.payer_vars.PR as f64)
}

/** ## Deductions for Canada Pension Plan additional contributions for the pay period.
*
*    NOTE, A separate formula is used for non-commissionable earnings.
*
*
*
* ### Arguements:
*
*   ctx: Context
*
*   [C](../other_deductions/fn.C.html): Canada (or Quebec) Pension Plan contributions for the pay period
*
*   [C2](../other_deductions/fn.C2.html): Second additional Canada (or Quebec) Pension Plan contributions for the pay period
*
* ### Examples:
*   //TODO: create examples...
*/
#[allow(non_snake_case)]
pub fn F5(ctx: Context, C: f64, C2: f64) -> f64 {
    let cpp_er_ee_ttl_cont_rate = ctx.tax_consts.CPP.TtlCPP_CRA.EE_ER_TtlContRate;
    let cpp_ee_er_add_cont_rate = ctx.tax_consts.CPP.CPPFAddtnlRate.EE_ER_FAddtnlContRate;
    if C == 0.0 && C2 == 0.0 {
        return 0.0;
    }
    utils::round(C * (cpp_ee_er_add_cont_rate / cpp_er_ee_ttl_cont_rate) + C2)
}

/** ## Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income
*
*
* ### Arguements:
*
*  F5: Deductions for Canada Pension Plan additional contributions for the pay period
*
* Use F5Q inplace of F5 for Quebec: Deductions for Quebec Pension Plan additional contributions for the pay period
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
*   B: Gross bonus, retroactive pay increase, vacation pay when vacation is not taken, accumulated overtime payment or other non-periodic payment
*
* ### Examples:
*   //TODO: create examples...
*/
#[allow(non_snake_case)]
pub fn F5A(F5: f64, PI: f64, B: f64) -> f64 {
    utils::round(F5 * ((PI - B) / PI))
}

/** ## Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the non-periodic payment
*
* ### Arguements:
*
* ctx: Context
*
*   [F5](./fn.F5.html): Deductions for Canada Pension Plan additional contributions for the pay period
*
*   Use F5Q inplace of F5 for Quebec: Deductions for Quebec Pension Plan additional contributions for the pay period
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable
*
* ### Examples:
*   //TODO: create examples...
*/
#[allow(non_snake_case)]
pub fn F5B(ctx: Context, F5: f64, PI: f64) -> f64 {
    utils::round(F5 * (ctx.payer_vars.B / PI))
}

/** ## Annual Basic Federal Tax
*
*   For cumulative T3 Calculations, use /[x/]_grad in the below list (if not listed, use the normal
*   parameter).
*
* ### Arguments:
*
*   ctx: Context
*
*   [A](../basic_personal_income/fn.A.html) or [A_grad](../basic_personal_income/fn.A_grad.html): Annual taxable income
*
*   [K1](fn.K1.html): Federal non-refundable personal tax credit (the lowest federal tax rate is used to calculate this credit)
*
*   [K2](fn.K2.html) or [K2_grad](fn.K2_grad.html): Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year (the lowest federal tax rate is used to calculate this credit).
*
*   Replace K2 with K2R where: employees that are transferred from Quebec to a location outside Quebec (currently unimplemented)
*
*   [K3](fn.K3.html): Other federal non-refundable tax credits (such as medical expenses and charitable donations) authorized by a tax services office or tax centre
*
*   [K4](fn.K4.html): Federal non-refundable tax credit calculated using the Canada employment amount (the lowest federal tax rate is used to calculate this credit)
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn T3(ctx: Context, A: f64, K1: f64, K2: f64, K3: f64, K4: f64) -> Result<f64, anyhow::Error> {
    let federal_threshold = ctx.tax_consts.fed.RITC.get_income_threshold(A)?;
    let result: f64 = (federal_threshold.R * A) - federal_threshold.K - K1 - K2 - K3 - K4;
    if result.is_sign_negative() {
        return Ok(0.0);
    }
    Ok(utils::round(result))
}

/** ## Federal non-refundable personal tax credit (the lowest federal tax rate is used to calculate this credit)
*
*
* ### Arguments:
*
*   ctx: Context
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn K1(ctx: Context) -> f64 {
    utils::round(ctx.tax_consts.fed.RITC.R[0] * ctx.payer_vars.TC)
}

/** ## Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year
*
*
* ### Arguments:
*
*   ctx: Context
*
*   [C](../other_deductions/fn.C.html): Canada (or Quebec) Pension Plan contributions for the pay period
*
*   [EI](../other_deductions/fn.EI.html): Insurable earnings for the pay period, including insurable taxable benefits for the pay period
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn K2(ctx: Context, C: f64, mut EI: f64) -> f64 {
    let cpp_max_contributions = ctx.tax_consts.CPP.BaseCPPRate.MaxEE_ER_TtlCont;

    let cpp_er_ee_ttl_cont_rate = ctx.tax_consts.CPP.TtlCPP_CRA.EE_ER_TtlContRate;
    let cpp_er_ee_base_cont_rate = ctx.tax_consts.CPP.BaseCPPRate.EE_ER_BaseContRate;
    let cpp_remainder_rate = cpp_er_ee_base_cont_rate / cpp_er_ee_ttl_cont_rate;

    let ei_max_prem = ctx.tax_consts.EI.MaxAEEP;

    if EI > ei_max_prem {
        EI = ei_max_prem;
    }

    let mut result =
        ctx.tax_consts.fed.RITC.R[0] * (ctx.payer_vars.P as f64 * C * cpp_remainder_rate);
    //TODO: check if the `result` is anywhere near CPP_MAX_CONTRIBUTIONS; not sure if I've writen
    //this correctly
    if result > cpp_max_contributions {
        result = cpp_max_contributions;
    }

    result = (result * (ctx.payer_vars.PM / 12) as f64)
        + (ctx.tax_consts.fed.RITC.R[0] * (ctx.payer_vars.P as f64 * EI));

    utils::round(result)
}

/** ## Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year
*
*   Using Cumulative Average Calculation
*
* ### Arguements:
*
*   [S1](../basic_personal_income/fn.S1.html): Annualizing factor
*
*   PI: Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period
*
*   [C](../other_deductions/fn.C.html): Canada (or Quebec) Pension Plan contributions for the pay period
*
*   [EI](../other_deductions/fn.EI.html): Insurable earnings for the pay period, including insurable taxable benefits for the pay period
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn K2_grad(ctx: Context, S1: f64, PI: f64, EI: f64) -> f64 {
    let mut cpp: f64;

    let cpp_max_contributions = ctx.tax_consts.CPP.BaseCPPRate.MaxEE_ER_TtlCont;
    let cpp_er_ee_base_cont_rate = ctx.tax_consts.CPP.BaseCPPRate.EE_ER_BaseContRate;
    let cpp_basic_excemption = ctx.tax_consts.CPP.TtlCPP_CRA.BasicException;

    cpp = (S1 * (PI + ctx.payer_vars.PIytd)) + ctx.payer_vars.B1 - cpp_basic_excemption;
    if cpp.is_sign_negative() {
        cpp = 0.0;
    }

    if cpp > cpp_max_contributions {
        cpp = cpp_max_contributions;
    }

    let mut result: f64;

    result = ctx.tax_consts.fed.RITC.R[0] * cpp_er_ee_base_cont_rate * cpp;

    let mut ei: f64;

    ei = (S1 * (EI + ctx.payer_vars.EIytd)) + ctx.payer_vars.B1;

    let ei_max_prem = ctx.tax_consts.EI.MaxAEEP;
    let ei_ee_cont_rate = ctx.tax_consts.EI.EE_CR;

    if ei > ei_max_prem {
        ei = ei_max_prem;
    }

    result += ctx.tax_consts.fed.RITC.R[0] * ei_ee_cont_rate * ei;

    utils::round(result)
}

/** ## Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year
*
*   Calculated using the year-to-date method
*
*
* ### Arguements:
*
*   ctx: Context
*
*   [C](../other_deductions/fn.C.html): Canada (or Quebec) Pension Plan contributions for the pay period
*
*   [EI](../other_deductions/fn.EI.html): Employment insurance premiums for the pay period
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn K2_YTD(ctx: Context, C: f64, EI: f64) -> f64 {
    let pm = ctx.payer_vars.PM;
    let pr = ctx.payer_vars.PR;
    let d = ctx.payer_vars.D;
    let d1 = ctx.payer_vars.D1;

    let cpp_max_contributions = ctx.tax_consts.CPP.BaseCPPRate.MaxEE_ER_TtlCont;

    let cpp_er_ee_ttl_cont_rate = ctx.tax_consts.CPP.TtlCPP_CRA.EE_ER_TtlContRate;
    let cpp_er_ee_base_cont_rate = ctx.tax_consts.CPP.BaseCPPRate.EE_ER_BaseContRate;
    let cpp_remainder_rate = cpp_er_ee_base_cont_rate / cpp_er_ee_ttl_cont_rate;

    let mut result: f64 = ctx.tax_consts.fed.RITC.R[0];
    let cpp_ftc1: f64 = cpp_max_contributions * (pm / 12) as f64;
    let cpp_ftc2: f64 = (d * cpp_remainder_rate) + (pr as f64 * C * cpp_remainder_rate);
    if cpp_ftc1 > cpp_ftc2 {
        result *= cpp_ftc2
    } else {
        result *= cpp_ftc1
    }

    let ei_ftc: f64;
    let ei_max_prem = ctx.tax_consts.EI.MaxAEEP;
    let y: f64 = d1 + (pr as f64 * EI);
    if y > ei_max_prem {
        ei_ftc = ei_max_prem;
    } else {
        ei_ftc = y;
    }

    result += ctx.tax_consts.fed.RITC.R[0] * ei_ftc;
    utils::round(result)
}

/** ## Other federal non-refundable tax credits
*
*
* ### Arguments:
*
*   ctx: Context
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn K3(ctx: Context) -> f64 {
    let p = ctx.payer_vars.P;
    let k3p = ctx.payer_vars.K3P;
    let pr = ctx.payer_vars.PR;
    match k3p {
        Some(k3p) => (p as f64 * k3p) / pr as f64,
        None => 0.0,
    }
}

/** ## Federal non-refundable tax credit calculated using the Canada employment amount (the lowest federal tax rate is used to calculate this credit)
*
*
* ### Arguements:
*
*   [A](.basic_personal_income/fn.A.html) or [A-Grad](.basic_personal_income/fn.A_grad.html): Annual taxable income
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn K4(ctx: Context, A: f64) -> Result<f64, anyhow::Error> {
    let cea = ctx
        .tax_consts
        .fed
        .ORA
        .Federal
        .CEA
        .ok_or_else(|| anyhow::anyhow!("Unable to find Federal CEA."))?;
    let k41: f64 = 0.15 * A;
    let k42: f64 = 0.15 * cea;
    if k41 > k42 {
        return Ok(utils::round(k42));
    } else {
        return Ok(utils::round(k41));
    }
}

/** ## Annual federal tax deduction
*
*
* ### Arguments:
*
*   ctx: Context
*
*   [T3](.fn.T3.html): Annual basic federal tax
*
*   [LCF](.fn.LCF.html): Federal labour-sponsored funds tax credit
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn T1(ctx: Context, T3: f64, LCF: f64) -> Result<f64, anyhow::Error> {
    let t1: f64;

    let P = ctx.payer_vars.P;
    let surtax = ctx
        .tax_consts
        .fed
        .ORA
        .notCA
        .Surtax
        .ok_or_else(|| anyhow::anyhow!("Unable to find Federal Surtax."))?;

    if ctx.payer_vars.lives_outside_city_limits {
        t1 = T3 + (surtax * T3) - (P as f64 * LCF);
    } else {
        t1 = T3 - (P as f64 * LCF);
    }

    if t1.is_sign_negative() {
        return Ok(0.0);
    }
    Ok(utils::round(t1))
}

/** ## Annual federal tax deduction
*
*   Uses Cumulative Average calculation
*
* ### Arguments:
*
*   ctx: Context
*
*   [T3](.fn.T3.html): Annual basic federal tax
*
*   [LCF](.fn.LCF.html): Federal labour-sponsored funds tax credit
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn T1_grad(ctx: Context, T3: f64, LCF: f64) -> Result<f64, anyhow::Error> {
    let t1: f64;
    let surtax = ctx
        .tax_consts
        .fed
        .ORA
        .notCA
        .Surtax
        .ok_or_else(|| anyhow::anyhow!("Unable to find Federal Surtax."))?;

    if ctx.payer_vars.lives_outside_city_limits {
        t1 = T3 + (surtax * T3) - LCF;
    } else {
        t1 = T3 - LCF;
    }

    if t1.is_sign_negative() {
        return Ok(0.0);
    }
    Ok(utils::round(t1))
}

/** ## Federal labour-sponsored funds tax credit
*
*
* ### Arguments:
*
*   ctx: Context
*
*   acquisition_pay_loss: Fifteen percent of the amount deducted or withheld for the pay period for the acquisition, by the employee, of approved shares of the capital stock of a prescribed labour-sponsored venture capital corporation
*
* ### Examples:
* TODO: Add examples...
*/
#[allow(non_snake_case)]
pub fn LCF(ctx: Context, acquisition_pay_loss: f64) -> Result<f64, anyhow::Error> {
    let fed_lcp_rate = ctx
        .tax_consts
        .fed
        .ORA
        .Federal
        .LCPRate
        .ok_or_else(|| anyhow::anyhow!("Unable to find Federal LCP Rate."))?;
    let fed_lcp_amt = ctx
        .tax_consts
        .fed
        .ORA
        .Federal
        .LCPAmt
        .ok_or_else(|| anyhow::anyhow!("Unable to find Federal LCP Amount."))?;
    let lcf: f64 = fed_lcp_rate * acquisition_pay_loss;
    if fed_lcp_amt > lcf {
        return Ok(utils::round(acquisition_pay_loss));
    } else {
        return Ok(fed_lcp_amt);
    }
}

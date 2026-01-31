//! # Formulas to calculate the estimated federal and provincial or territorial tax deductions (T) for the pay period

use crate::context::Context;
use crate::utils;

/// ## Estimated federal and provincial or territorial tax deductions for the pay period
///   (Non-Commissionable earnings)
///
///
/// ### Arguements:
///
///   ctx: Context
///
///   T1: Annual federal tax deduction
///
///   T2: Annual provincial or territorial tax deduction (except Quebec)
///
/// ### Examples:
/// //TODO: create examples.
#[allow(non_snake_case)]
pub fn T(ctx: Context, T1: f64, T2: f64,) -> f64 {
    let l = match ctx.payer_vars.L {
        Some(l,) => l,
        None => 0.0,
    };
    utils::round(((T1 + T2) / ctx.payer_vars.P as f64) + l,)
}

/// ## Estimated Federal and Provincial or Territorial Tax Deductions for the Pay Period
///
///   Uses Cumulative Average Calculation
///
/// ### Arguements:
///
///   ctx: Context
///
///   T1_grad: Annual federal tax deduction (Uses cumalitve average calculation)
///
///   T2: Annual provincial or territorial tax deduction (except Quebec)
///
///   S1: Annualizing factor
///
/// ### Examples:
/// //TODO: create examples
#[allow(non_snake_case)]
pub fn T_grad(ctx: Context, T1_grad: f64, T2: f64, S1: f64,) -> f64 {
    let t: f64;
    let l = match ctx.payer_vars.L {
        Some(l,) => l,
        None => 0.0,
    };

    t = ((T1_grad + T2 - ctx.payer_vars.M1) / S1) - ctx.payer_vars.M;
    if t.is_sign_negative() {
        return l;
    }

    utils::round(t + l,)
}

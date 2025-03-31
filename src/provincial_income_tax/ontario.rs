//! # Ontario Provincial Income Tax

use crate::context::Context;
use crate::utils;

/** ## Provincial surtax calculated on the basic provincial tax (only applies to Ontario)
*
*
* ### Arguements:
*
*   [ctx](../../context/struct.Context.html): Context
*
*   [T4](../provincial_income_tax/fn.T4.html): Annual basic provincial or territorial tax
*
* ### Example:
*
* ```
* use cdn_payroll::context::{Context, Version, ProvinceKey};
* use cdn_payroll::provincial_income_tax::ontario::V1;
*
* let result = Context::new(Version::V2025_1, ProvinceKey::ON, 16129.0, 52, 51, 12, 0, None, None,
* None, None, None, false, None, None);
* assert!(result.is_ok());
*
* let ctx = result.unwrap();
*
* let t4 = 5400.0;
* let v1 = V1(&ctx, &t4);
*
* assert_eq!(v1, Ok(0.0));
*
* let t4 = 6770.0;
* let v1 = V1(&ctx, &t4);
*
* assert_eq!(v1, Ok(212.0));
*
* let t4 = 8000.0;
* let v1 = V1(&ctx, &t4);
*
* assert_eq!(v1, Ok(707.48));
* ```
*/
#[allow(non_snake_case)]
pub fn V1(ctx: &Context, T4: &f64) -> Result<f64, &'static str> {
    let ctx_ora_on = &ctx.tax_constants.prov.ORA;

    let t4atv1 = &ctx_ora_on
        .T4atV1
        .as_ref()
        .ok_or_else(|| "unable to locate V1 at T4[x].")?;
    let v1 = &ctx_ora_on
        .V1Rate
        .as_ref()
        .ok_or_else(|| "unable to locate V1 Rates for T4.")?;

    let t4atv1_1 = t4atv1
        .get(1)
        .ok_or_else(|| "unable to locate V1 at T4[x] (level 1).")?;
    let v1_1 = v1
        .get(1)
        .ok_or_else(|| "unable to locate V1 Rates for T4 (level 1).")?;

    let t4atv1_2 = t4atv1
        .get(2)
        .ok_or_else(|| "unable to locate V1 at T4[x] (level 2).")?;
    let v1_2 = v1
        .get(2)
        .ok_or_else(|| "unable to locate V1 Rate (level 2) for T4.")?;

    Ok(utils::round(match T4 {
        T4 if T4 <= t4atv1_1 => 0.0,
        T4 if T4 <= t4atv1_2 => v1_1 * (T4 - t4atv1_1),
        _ => v1_1 * (T4 - t4atv1_1) + (v1_2 * (T4 - t4atv1_2)),
    }))
}

/** ## Additional tax calculated on taxable income (only applies to the Ontario Health Premium)
*
*
* ### Arguements:
*
*   [ctx](../../context/struct.Context.html): Context
*
*   [A](../../basic_personal_income/fn.A.html): Annual taxable income
*
* ### Example:
*
* ```
* use cdn_payroll::context::{Context, Version, ProvinceKey};
* use cdn_payroll::provincial_income_tax::ontario::V2;
*
* let result = Context::new(Version::V2025_1, ProvinceKey::ON, 16129.0, 52, 51, 12, 0, None, None,
* None, None, None, false, None, None);
* assert!(result.is_ok());
*
* let ctx = result.unwrap();
*
* let a = 20000.0;
* let v2 = V2(&ctx, &a);
* assert_eq!(v2, Ok(0.0));
*
* let a = 36000.0;
* let v2 = V2(&ctx, &a);
* assert_eq!(v2, Ok(300.0));
*
* let a = 200500.0;
* let v2 = V2(&ctx, &a);
* assert_eq!(v2, Ok(875.0));
*
* let a = 72500.0;
* let v2 = V2(&ctx, &a);
* assert_eq!(v2, Ok(725.0));
* ```
*/
#[allow(non_snake_case)]
pub fn V2(ctx: &Context, A: &f64) -> Result<f64, &'static str> {
    let mut v2: f64 = 0.0;
    let ctx_ora_on = &ctx.tax_constants.prov.ORA;
    let aatv2 = &ctx_ora_on
        .AatV2
        .as_ref()
        .ok_or_else(|| "unable to locate A to V2.")?;
    let v2_rate = &ctx_ora_on
        .V2Rate
        .as_ref()
        .ok_or_else(|| "unable to locate V2 rate.")?;
    let v2_max = &ctx_ora_on
        .V2Max
        .as_ref()
        .ok_or_else(|| "unable to locate V2 Maximum contributions.")?;

    for (i, v2_rate_i) in v2_rate.iter().enumerate() {
        let aatv2_i = match aatv2.get(i) {
            Some(x) => x,
            None => &(A),
        };

        match aatv2_i {
            aatv2_i if A <= aatv2_i => match i {
                i if i == 0 => {
                    break;
                }
                _ => {
                    let v2_max_i = v2_max[i];
                    let v2_max_in1 = v2_max[i - 1];
                    let aatv2_in1 = aatv2[i - 1];
                    v2 = utils::round(match v2_max_in1 + (v2_rate_i * (A - aatv2_in1)) {
                        v2 if v2 < v2_max_i => v2,
                        _ => v2_max_i,
                    });
                    break;
                }
            },
            _ => {}
        }
    }

    Ok(v2)
}

/** ## Provincial tax reduction (only applies to Ontario and British Columbia)
*
*
* ### Arguements:
*
*   [ctx](../../context/struct.Context.html): Context
*
*   [T4](../provincial_income_tax/fn.T4.html): Annual basic provincial or territorial tax
*
*   [V1](fn.V1.html): Provincial surtax calculated on the basic provincial tax (only applies to Ontario)
*
*   [Y](fn.Y.html): Additional provincial tax reduction amount based on the number of eligible dependents used in the calculation of Factor S (only applies to Ontario)
*
* ### Example:
* ```
* use cdn_payroll::context::{Context, Version, ProvinceKey};
* use cdn_payroll::provincial_income_tax::ontario::{V1, S};
*
* let result = Context::new(Version::V2025_1, ProvinceKey::ON, 16129.0, 52, 51, 12, 0, None, None,
* None, None, None, false, None, None);
* assert!(result.is_ok());
*
* let ctx = result.unwrap();
*
* let t4 = 5000.0;
* let v1 = V1(&ctx, &t4);
*
* if let Ok(v1) = v1 {
*   let s = S(&ctx, &t4, &v1, None);
*   assert_eq!(s, Ok(0.0));
*
*   let y = 3264.0;
*   let s = S(&ctx, &t4, &v1, Some(&y));
*   assert_eq!(s, Ok(2116.0));
* }
*
*
* ```
*/
#[allow(non_snake_case)]
pub fn S(ctx: &Context, T4: &f64, V1: &f64, Y: Option<&f64>) -> Result<f64, &'static str> {
    let s2 = &ctx.tax_constants.prov.ORA.S2.ok_or_else(|| "unable to locate S2.")?;
    let y = match Y {
        Some(y) => y,
        None => &0.0,
    };

    Ok(utils::round({
        let s = match ((T4 + V1), ((2.0 * (s2 + &(*y as f64))) - (T4 + V1))) {
            (a, b) => {
                if a > b {
                    b
                } else {
                    a
                }
            }
        };
        match s {
            s => {
                if s > 0.0 {
                    s
                } else {
                    0.0
                }
            }
        }
    }))
}

/** ## Additional provincial tax reduction amount based on the number of eligible dependents used in the calculation of Factor S (only applies to Ontario)
*
*
* ### Arguements:
*
*   [ctx](../../context/struct.Context.html): Context
*
*   number_of_disabled_dependants: Number of disabled dependants
*
*   number_of_minor_dependents: Number of dependents under the age of 19
*
* ### Example:
* ```
*   use cdn_payroll::provincial_income_tax::ontario::Y;
*   use cdn_payroll::context::{Context, Version, ProvinceKey};
*
* let r = Context::new(Version::V2025_1, ProvinceKey::ON, 16129.0, 52, 51, 12, 0, None, None,
* None, None, None, false, None, None);
*   assert!(r.is_ok());
*
*   let ctx = r.unwrap();
*   let dd = 1;
*   let md = 1;
*
*   let y = Y(&ctx, &dd, &md);
*
*   assert_eq!(y, Ok(544.0+544.0));
* ```
*/
#[allow(non_snake_case)]
pub fn Y(
    ctx: &Context,
    number_of_disabled_dependants: &i64,
    number_if_minor_dependents: &i64,
) -> Result<f64, &'static str> {
    let ora_on = &ctx.tax_constants.prov.ORA;
    let y_factor = &ora_on.YFactor.ok_or_else(|| "unable to locate Y factor.")?;

    Ok(y_factor * &(*number_of_disabled_dependants as f64)
        + y_factor * &(*number_if_minor_dependents as f64))
}

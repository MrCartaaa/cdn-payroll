//! # Basic Personal Amount Calculation
//! The Basic Personal Amount (BPA) is a non-refundable tax credit that all individuals can claim in Canada. It provides a full reduction from federal income tax for individuals with taxable income below the BPA and a partial reduction for those with taxable income above it. 
//! It's important to note that the BPA is adjusted annually due to inflation and government policy.

use crate::utils;

mod v2025 {

    pub const INCOME_THRESHOLD_4: f64 = 177882.0;
    pub const INCOME_THRESHOLD_5: f64 = 253414.0;
    pub const CPP_CONTRIBUTION_MAX: f64 = 1591.0;
    pub const MINIMUM_BASIC_AMT: f64 = 16129.0;
    pub const MAXIMUM_BASIC_AMT: f64 = 14538.0;
}

/** Calculate Federal Basic Personal Amount.
*
*
* Given:
*
*   A: Annual Taxable Income
*
*   HD: Annual deduction for living in a prescribed zone, as shown on Form TD1
*
* Where:
*
*   NI: Net Income
*
*   NI = A + HD
*/
#[allow(non_snake_case)]
pub fn BPAF(A: f64, HD: f64) -> Result<f64, f64> {
    let mut BPAF: f64 = 0.0;
    let NI = A+HD;

    if NI <= v2025::INCOME_THRESHOLD_4 {
        BPAF = v2025::MINIMUM_BASIC_AMT;
    } else
    if v2025::INCOME_THRESHOLD_4 < NI && NI < v2025::INCOME_THRESHOLD_5 {
        BPAF = v2025::MINIMUM_BASIC_AMT - (NI*-v2025::INCOME_THRESHOLD_4) * (v2025::CPP_CONTRIBUTION_MAX / 75532.0);
    } else
    if NI > v2025::INCOME_THRESHOLD_5 {
        BPAF = v2025::MAXIMUM_BASIC_AMT;
    }

    if BPAF == 0.0 {
        return Err(0.0)
    }

    Ok(utils::round(BPAF))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_BPAF_minimum_amt() {
        let result = BPAF(10000.0, 0.0);
        assert_eq!(result.unwrap(), v2025::MINIMUM_BASIC_AMT);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BPAF_maximum_amt() {
        let result = BPAF(253414.01, 0.0);
        assert_eq!(result.unwrap(), v2025::MAXIMUM_BASIC_AMT);
    }

}


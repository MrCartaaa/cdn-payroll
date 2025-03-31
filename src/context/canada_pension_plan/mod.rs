//! Canada Pension Plan Rates and Amounts.

mod base_cpp_rates_and_amounts;
mod ttl_cpp_cont_rates_amts;
mod first_additional_cpp_rates_and_amounts;
mod second_additional_cpp_rates_and_amounts;

use crate::context::{Version, ProvinceKey};
use std::error::Error;
pub use base_cpp_rates_and_amounts::*;
pub use ttl_cpp_cont_rates_amts::*;
pub use first_additional_cpp_rates_and_amounts::*;
pub use second_additional_cpp_rates_and_amounts::*;



/// Context for CPP Constants
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CPPCtx {
    pub TtlCPP_CRA: TtlCPP_CRA,
    pub BaseCPPRate: BCPPRates,
    pub CPPFAddntlRate: FACPP_RA,
    pub CPPSAddntlRate: SACPP_RA,
}

impl CPPCtx {

    #[allow(non_snake_case)]
    pub fn new(version: &Version, prov: &ProvinceKey) -> Result<Self, Box<dyn Error>> {
        let TtlCPP_CRA = TtlCPP_CRA::init(&version, &prov)?;
        let BaseCPPRate = BCPPRates::init(&version, &prov)?;
        let CPPFAddntlRate = FACPP_RA::init(&version, &prov)?;
        let CPPSAddntlRate = SACPP_RA::init(&version, &prov)?;
        Ok(CPPCtx {
                TtlCPP_CRA,
                BaseCPPRate,
                CPPFAddntlRate,
                CPPSAddntlRate
            })
    }
}

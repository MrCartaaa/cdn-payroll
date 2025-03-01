#![doc=include_str!("../README.md")]

/// Basic Personal Income
pub mod basic_personal_income;

/// Income Taxes
pub mod federal_income_tax;
pub mod income_tax;
pub mod provincial_income_tax;

/// CPP & EI Deductions
pub mod other_deductions;

/// Context: defined annual constants for each year
pub mod context;

#[doc(hidden)]
pub mod utils;

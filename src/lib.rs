#![deny(clippy::disallowed_methods)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

#![doc=include_str!("../README.md")]

pub mod basic_personal_income;

pub mod federal_income_tax;
pub mod income_tax;
pub mod provincial_income_tax;

pub mod other_deductions;

pub mod context;

#[doc(hidden)]
pub mod utils;

# Canadian Payroll Crate

Based on the [Payroll Deductions Formulas, 120th Edition](https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan/t4127-jan-payroll-deductions-formulas-computer-programs.html)

--- 

## Current Implementation
- There is no implementation for provincial taxes other than Ontario
- There is incomplete implementation for taxes on commissionable and non-periodic payment earnings
- unit testing incomplete
- the crate is not mature enough for integration tests, but this will be implemented during that phase
- calculations currently only work for CY 2025

## 2025 Q1/Q2 Road Map
1. ~define cumulative deductions~
2. ~define CPP and EI calculations~
3. ~identify constants by year and use them in the functions
    This will be done by adding a ctx param into each function that will identify the date/year of the request, and the constants identified from there.
    These constants will be extracted from the [CRA provided CSV files](https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan.html)~
4. complete unit testing
5. implement provincial tax calculations
6. implement commissionable & non-periodic earnings tax calculations



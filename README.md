# Canadian Payroll Crate

Based on the [Payroll Deductions Formulas, 120th Edition](https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan.html)

--- 

## Current Implementation
- There is no implementation for provincial taxes other than Ontario
- There is incomplete implementation for taxes on commissionable and non-periodic payment earnings
- certain values are hard coded into the function as I have yet to find out where their origin is; otherwise, they are defined as constants, by year
- unit testing incomplete
- the crate is not mature enough for integration tests, but this will be implemented during that phase
- calculations currently only work for CY 2025

## 2025 Q1 Road Map
3. complete unit testing
4. identify constants by year and use them in the functions
    This will be done by adding a ctx param into each function that will identify the date/year of the request, and the constants identified from there.
    These constants will be extracted from the [CRA provided CSV files](https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan.html)
5. implement provincial tax calculations
6. implement commissionable and non-periodic payment earnings tax calculations


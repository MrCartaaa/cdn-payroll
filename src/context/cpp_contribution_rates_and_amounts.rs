//! Canada Pension Plan / Quebec Pension Plan Contribution Rates and Amounts as defined by the CRA.

use std::error::Error as StdError;
use serde::{Serialize, Deserialize, Deserializer, de};
use csv::{ReaderBuilder, Error as CSVError};
use serde_json::Value;
use super::Version;

/** Canada Pension Plan / Quebec Pension Plan Contribution Rates and Amounts for Quebec and
* Non-Quebec Individuals
*
* Where:
*
*   CA: Individuals living in Canada, outside of Quebec
*
*   QC: Individuals living in Quebec
*/
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CanadaPensionPlanContributionRatesAndAmounts {
    pub CA: CPP_CRA,
    pub QC: CPP_CRA,
}

impl CanadaPensionPlanContributionRatesAndAmounts {

    /** Initialize Canada Pension Plan / Quebec Pension Plan Contribution Rates and Amounts.
    */
    pub fn init(version: &Version) -> Result<CanadaPensionPlanContributionRatesAndAmounts, Box<dyn StdError>> {

        let file_name = match version {
            Version::V2025_1 => {"cra-constants/v2025_1/cpp-qpp-ttl-01-25e.csv"}
        };

        let rdr = ReaderBuilder::new().from_path(file_name)?;
        let mut records: Vec<CPP_CRA> = Vec::new();

        for result in rdr.into_deserialize() {
            let rec: Result<CPP_CRA, CSVError> = result;
            if rec.is_ok() {
                records.push(rec.unwrap().clone());
            }
        }

        if records.len() != 2 {
            return Err("Datafile Corrupt. expected 2 rows from {file_name}".into());
        }
 
        #[allow(non_snake_case)]
        if let Some(QC) = records.iter().position(|rec| rec.pp == "QPP (QC)") {
            #[allow(non_snake_case)]
            if let Some (CA) = records.iter().position(|rec| rec.pp == "CPP (Canada except QC)") {
                return Ok(
                    CanadaPensionPlanContributionRatesAndAmounts{
                        QC: records.get(QC).unwrap().clone(),
                        CA: records.get(CA).unwrap().clone(),
                    }
                )
            }
        }
        Err("Datafile Corrupt, expected values in columns from {file_name}.".into())
    }
}

/** Canada Pension Plan Contribution Rate Amounts
*
* Where:
*
*   YMPE: Years Maximum Pensionable Earnings
*
*   BasicException: Allowable Earnings before they are subject to Pension Contributions
*
*   YMCE: Years Maximum Contributory Earnings
*
*   EE_ER_TtlContRate: Employee and Employer Total Contribution Rate
*
*   MaxEE_ER_TtlCont: Maximum Employee and Employer Total Contribution
*
*   YMPE_raw: YMPE Before Rounding
*/
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CPP_CRA {
    #[serde(rename="CPP/QPP")]
    pp: String,
    #[serde(deserialize_with="quoted_f64", rename="Year's Maximum Pensionable Earnings (YMPE)")]
    pub YMPE: f64,
    #[serde(deserialize_with="quoted_f64", rename="Basic Exception")]
    pub BasicException: f64,
    #[serde(deserialize_with="quoted_f64", rename="Year's Maximum Contributory Earnings")]
    pub YMCE: f64,
    #[serde(deserialize_with="quoted_f64", rename="Base Employee and Employer Contribution Rate")]
    pub EE_ER_TtlContRate: f64,
    #[serde(deserialize_with="quoted_f64", rename="Maximum Base Employee and Employer Contribution*")]
    pub MaxEE_ER_TtlCont: f64,
    #[serde(deserialize_with="quoted_f64", rename="YMPE Before Rounding")]
    pub YMPE_raw: f64,
}

fn quoted_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.trim().replace(",", "");
            if v.is_empty() {
                return Err(de::Error::custom(format!("no string to parse: {}", s)));
            }
            let val = v.parse::<f64>();
            if val.is_ok() {
                return Ok(val.unwrap());
            } else {
                return Err(de::Error::custom(format!("{}, Val: {:?}", val.unwrap_err(), v)));
            }
                            },
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))?,
        _ => return Err(de::Error::custom("Wrong type, expected quoted f64.")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_init_cpp_const_rates() {
        let result = CanadaPensionPlanContributionRatesAndAmounts::init(&Version::V2025_1);
        assert!(result.is_ok());

        let cppcr = result.unwrap();
        assert_eq!(cppcr.CA.MaxEE_ER_TtlCont, 4034.1);
        assert_eq!(cppcr.QC.YMPE, 71300.0);

    }
}


//! First Additional Canada Pension Plan / Quebec Pension Plan Rates and Amounts as defined by the CRA.

use crate::context::Version;
use csv::{Error as CSVError, ReaderBuilder};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;

/** First Additional Canada Pension Plan / Quebec Pension Plan Rates and Amounts for Quebec and Non-Quebec
* Individuals
*
* Where:
*
*   CA: Individuals living in Canada, outside of Quebec
*
*   QC: Individuals living in Quebec
*/
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct FirstAdditionalCanadaPensionPlanRatesAndAmounts {
    pub CA: FACPP_RA,
    pub QC: FACPP_RA,
}

impl FirstAdditionalCanadaPensionPlanRatesAndAmounts {
    /// Initialize Canada Pension Plan / Quebec Pension Plan Rates and Amounts.
    pub fn init(
        version: &Version,
    ) -> Result<FirstAdditionalCanadaPensionPlanRatesAndAmounts, Box<dyn StdError>> {
        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/cpp-qpp-addntl-01-25e.csv",
        };

        let rdr = ReaderBuilder::new().from_path(file_name)?;
        let mut records: Vec<FACPP_RA> = Vec::new();

        for result in rdr.into_deserialize() {
            let rec: Result<FACPP_RA, CSVError> = result;
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
            if let Some(CA) = records
                .iter()
                .position(|rec| rec.pp == "CPP (Canada except QC)")
            {
                return Ok(FirstAdditionalCanadaPensionPlanRatesAndAmounts {
                    QC: records.get(QC).unwrap().clone(),
                    CA: records.get(CA).unwrap().clone(),
                });
            }
        }
        Err("Datafile Corrupt, expected values in columns from {file_name}.".into())
    }
}

/** First Additional Canada Pension Plan Rates & Amounts
*
* Where:
*
*   YMPE: Years Maximum Pensionable Earnings
*
*   EE_ER_FAddtnlContRate: Employee and Employer First Additional Contribution Rate
*
*   MaxEE_ER_TFAddtlCont: Maximum Employee and Employer First Additional Contribution
*/
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FACPP_RA {
    #[serde(rename = "CPP/QPP")]
    pp: String,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Year's Maximum Pensionable Earnings (YMPE)"
    )]
    pub YMPE: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "First Additional Employee and Employer Contribution Rate"
    )]
    pub EE_ER_FAddtnlContRate: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Maximum First Additional Employee and Employer Contribution*"
    )]
    pub MaxEE_ER_FAddtnlCont: f64,
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
                return Err(de::Error::custom(format!(
                    "{}, Val: {:?}",
                    val.unwrap_err(),
                    v
                )));
            }
        }
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))?,
        _ => return Err(de::Error::custom("Wrong type, expected quoted f64.")),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_cpp_faddtl_rate() {
        let result = FirstAdditionalCanadaPensionPlanRatesAndAmounts::init(&Version::V2025_1);
        assert!(result.is_ok());

        let bcppra = result.unwrap();
        assert_eq!(bcppra.QC.YMPE, 71300.0);
        assert_eq!(bcppra.QC.MaxEE_ER_FAddtnlCont, 678.0);
        assert_eq!(bcppra.CA.EE_ER_FAddtnlContRate, 0.0100);
    }
}

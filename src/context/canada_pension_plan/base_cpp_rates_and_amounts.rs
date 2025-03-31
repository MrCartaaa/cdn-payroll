//! Base Canada Pension Plan / Quebec Pension Plan Rates and Amounts as defined by the CRA.

use crate::context::{ProvinceKey, Version};
use csv::{Error as CSVError, ReaderBuilder};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;

/** Base Canada Pension Plan Rates & Amounts
*
* Where:
*
*   YMPE: Years Maximum Pensionable Earnings
*
*   EE_ER_BaseContRate: Employee and Employer Base Contribution Rate
*
*   MaxBaseEE_ER_TtlCont: Maximum Employee and Employer Base Contribution
*/
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BCPPRates {
    #[serde(rename = "CPP/QPP")]
    pp: String,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Year's Maximum Pensionable Earnings (YMPE)"
    )]
    pub YMPE: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Base Employee and Employer Contribution Rate"
    )]
    pub EE_ER_BaseContRate: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Maximum Base Employee and Employer Contribution*"
    )]
    pub MaxEE_ER_TtlCont: f64,
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

impl BCPPRates {
    /// Initialize Base Canada Pension Plan / Quebec Pension Plan Rates and Amounts.
    pub fn init(
        version: &Version,
        prov: &ProvinceKey
    ) -> Result<BCPPRates, Box<dyn StdError>> {
        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/cpp-qpp-br-01-25e.csv",
        };

        let rdr = ReaderBuilder::new().from_path(file_name)?;
        let mut records: Vec<BCPPRates> = Vec::new();

        for result in rdr.into_deserialize() {
            let rec: Result<BCPPRates, CSVError> = result;
            if rec.is_ok() {
                records.push(rec.unwrap().clone());
            }
        }

        if records.len() != 2 {
            return Err("Datafile Corrupt. expected 2 rows from {file_name}".into());
        }
        
        Ok(match prov {
            ProvinceKey::QC => records.get(records.iter().position(|rec| rec.pp == "QPP (QC)").ok_or_else(|| "DataFile Corrupt. Count not find Base Quebec CPP Rates.")?).unwrap().to_owned(),
            _ => records.get(records.iter().position(|rec| rec.pp == "CPP (Canada except QC)").ok_or_else(|| "DataFile Corrupt. Could not find Base CPP Rates.")?).unwrap().to_owned()
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_cpp_base_rates() {
        let result = BCPPRates::init(&Version::V2025_1, &ProvinceKey::ON);
        assert!(result.is_ok());

        let bcppra = result.unwrap();
        assert_eq!(bcppra.EE_ER_BaseContRate, 0.0495);
 
        let result = BCPPRates::init(&Version::V2025_1, &ProvinceKey::ON);
        assert!(result.is_ok());

        let bcppra = result.unwrap();
        assert_eq!(bcppra.YMPE, 71300.0);
    }
}

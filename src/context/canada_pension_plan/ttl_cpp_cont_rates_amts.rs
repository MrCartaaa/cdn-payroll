//! Total Canada Pension Plan / Quebec Pension Plan Contribution Rates and Amounts as defined by the CRA.

use crate::context::{ProvinceKey, Version};
use csv::{Error as CSVError, ReaderBuilder};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;



/** Total Canada Pension Plan Contribution Rate Amounts
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
pub struct TtlCPP_CRA {
    #[serde(rename = "CPP/QPP")]
    pp: String,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Year's Maximum Pensionable Earnings (YMPE)"
    )]
    pub YMPE: f64,
    #[serde(deserialize_with = "quoted_f64", rename = "Basic Exception")]
    pub BasicException: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Year's Maximum Contributory Earnings"
    )]
    pub YMCE: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Base Employee and Employer Total Contribution Rate"
    )]
    pub EE_ER_TtlContRate: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Maximum Base Employee and Employer Contribution*"
    )]
    pub MaxEE_ER_TtlCont: f64,
    #[serde(deserialize_with = "quoted_f64", rename = "YMPE Before Rounding")]
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
impl TtlCPP_CRA {
    /** Initialize Total Canada Pension Plan / Quebec Pension Plan Contribution Rates and Amounts.
     */
    pub fn init(
        version: &Version,
        prov: &ProvinceKey,
) -> Result<Self, Box<dyn StdError>> {
        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/cpp-qpp-ttl-01-25e.csv",
        };

        let rdr = ReaderBuilder::new().from_path(file_name)?;
        let mut records: Vec<TtlCPP_CRA> = Vec::new();

        for result in rdr.into_deserialize() {
            let rec: Result<TtlCPP_CRA, CSVError> = result;
            if rec.is_ok() {
                records.push(rec.unwrap().clone());
            }
        }

        if records.len() != 2 {
            return Err("Datafile Corrupt. expected 2 rows from {file_name}".into());
        }

        Ok(match &prov {
            // TODO: I dont like how this is written
            ProvinceKey::QC => records.get(records.iter().position(|rec| rec.pp == "QPP (QC)").ok_or_else(|| "Datafile Corrupt. unable to find QC Total CPP Contributions details.")?).unwrap().clone(),
            _ => records.get(records.iter().position(|rec| rec.pp == "CPP (Canada except QC)").ok_or_else(|| "Datafile Corrupt. unable to find Canada CPP Contribution details.")?).unwrap().clone(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_cpp_const_rates() {
        let result = TtlCPP_CRA::init(&Version::V2025_1, &ProvinceKey::ON);
        assert!(result.is_ok());

        let cppcr = result.unwrap();
        assert_eq!(cppcr.MaxEE_ER_TtlCont, 4034.1);
        assert_eq!(cppcr.EE_ER_TtlContRate, 0.0595);

        let result = TtlCPP_CRA::init(&Version::V2025_1, &ProvinceKey::QC);
        assert!(result.is_ok());
        let qc_cppcr = result.unwrap();

        assert_eq!(qc_cppcr.YMPE, 71300.0);
    }
}

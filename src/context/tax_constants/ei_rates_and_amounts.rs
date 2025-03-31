//! Employment Insurance Rates and Amounts as defined by the CRA.

use super::{Version, ProvinceKey};
use csv::{Error as CSVError, ReaderBuilder};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;

impl EI_RA {
    /// Initialize Employment Insurance Rates and Amounts
    pub fn init(
        version: &Version,
        prov: &ProvinceKey,
    ) -> Result<EI_RA, Box<dyn StdError>> {
        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/ei-01-25e.csv",
        };

        let rdr = ReaderBuilder::new().from_path(file_name)?;
        let mut records: Vec<EI_RA> = Vec::new();

        for result in rdr.into_deserialize() {
            let rec: Result<EI_RA, CSVError> = result;
            if rec.is_ok() {
                records.push(rec.unwrap().clone());
            }
        }

        if records.len() != 2 {
            return Err("Datafile Corrupt. expected 2 rows from {file_name}".into());
        }

        Ok(match prov {
            ProvinceKey::QC => records.get(records.iter().position(|rec| rec.ei == "QC").ok_or_else(|| "Datafile Corrupt. Unable to find EI Rates for QC.")?).unwrap().to_owned(),
            _ => records.get(records.iter().position(|rec| rec.ei == "Canada except QC").ok_or_else(|| "Datafile Corrupt. Unable to find EI Rates.")?).unwrap().to_owned(),
        })
    }
}

/** Employment Insurance Rates and Amounts
*
* Where:
*
*   MaxAIE: Maximum Annual Insurable Earnings
*
*   EE_CR: Employee Contribution Rate
*
*   ER_CR: Employer Contrubition Rate
*
*   MaxAEEP: Maximum Annual Employee Premium
*
*   MaxAERP: Maximum Annual Employer Premium
*/
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EI_RA {
    #[serde(rename = "EI")]
    ei: String,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Maximum Annual Insurable Earnings"
    )]
    pub MaxAIE: f64,
    #[serde(deserialize_with = "quoted_f64", rename = "Employee Contribution Rate")]
    pub EE_CR: f64,
    #[serde(deserialize_with = "quoted_f64", rename = "Employer Contribution Rate")]
    pub ER_CR: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Maximum Annual Employee Premium"
    )]
    pub MaxAEEP: f64,
    #[serde(
        deserialize_with = "quoted_f64",
        rename = "Maximum Annual Employer Premium"
    )]
    pub MaxAERP: f64,
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
    fn test_init_cpp_base_rates() {
        let result = EI_RA::init(&Version::V2025_1, &ProvinceKey::QC);
        assert!(result.is_ok());

        let eira = result.unwrap();
        assert_eq!(eira.MaxAERP, 1204.94);
        assert_eq!(eira.MaxAIE, 65700.0);

        let result = EI_RA::init(&Version::V2025_1, &ProvinceKey::ON);
        assert!(result.is_ok());

        let eira = result.unwrap();
        assert_eq!(eira.EE_CR, 0.0164);
    }
}

//! Federal Claim Codes (using maimum BPAF) as defined by the CRA.

use crate::context::Version;
use csv::ReaderBuilder;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;

/** Federal Claim Code
*
* Where:
*
*   TCAmtFloor: Total Claim Amount Floor
*
*   TCAmtCeil: Total Claim Amount Ceiling
*
*   TC: Option 1 TC
*
*   K1: Option 1, K1
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct FederalClaimCode {
    #[serde(
        deserialize_with = "quoted_option_f64",
        rename = "Total claim amount ($) from"
    )]
    pub TCAmtFloor: Option<f64>,
    #[serde(
        deserialize_with = "quoted_option_f64",
        rename = "Total claim amount ($) to"
    )]
    pub TCAmtCeil: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "Option 1, TC ($)")]
    pub TC: f64,
    #[serde(deserialize_with = "quoted_f64", rename = "Option 1, K1 ($)")]
    pub K1: f64,
}
fn quoted_option_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    Ok(match Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.trim().replace(",", "");
            if v.is_empty() {
                return Err(de::Error::custom(format!("no string to parse: {}", s)));
            }
            let val = v.parse::<f64>();
            if val.is_ok() {
                return Ok(Some(val.unwrap()));
            } else {
                return Ok(None);
            }
        }
        Value::Number(n) => n.as_f64(),
        _ => return Err(de::Error::custom("Wrong type, expected quoted f64.")),
    })
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

pub trait FederalClaimCodesGetter {
    /// Initialize Federal Claim Codes
    fn init_cc(version: &Version) -> Result<Vec<FederalClaimCode>, Box<dyn StdError>> {
        let mut records: Vec<FederalClaimCode> = Vec::new();

        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/cc-fd-01-25e.csv",
        };

        let mut rdr = ReaderBuilder::new().from_path(file_name)?;

        for result in rdr.deserialize() {
            let rec: FederalClaimCode = result?;
            records.push(rec.clone());
        }

        if records.len() != 11 {
            return Err("Datafile Corrupt. Expected 11 claim codes (0-10)".into());
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_fed_claim_codes() {
        struct FederalClaimCodes {}
        impl FederalClaimCodesGetter for FederalClaimCodes {}

        let result = FederalClaimCodes::init_cc(&Version::V2025_1);
        assert!(result.is_ok());

        let fcc = result.unwrap();

        assert_eq!(
            fcc.get(0).unwrap(),
            &FederalClaimCode {
                TCAmtFloor: None,
                TCAmtCeil: None,
                TC: 0.0,
                K1: 0.0
            }
        );
        assert_eq!(
            fcc.get(5).unwrap(),
            &FederalClaimCode {
                TCAmtFloor: Some(24463.01),
                TCAmtCeil: Some(27241.0),
                TC: 25852.0,
                K1: 3877.8
            }
        );
        assert_eq!(
            fcc.get(9).unwrap(),
            &FederalClaimCode {
                TCAmtFloor: Some(35575.01),
                TCAmtCeil: Some(38353.0),
                TC: 36964.0,
                K1: 5544.6
            }
        );
    }
}

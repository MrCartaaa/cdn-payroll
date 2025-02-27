//! Ontario Claim Codes (using maimum BPAF) as defined by the CRA.

use std::error::Error as StdError;
use serde::{Serialize, Deserialize, Deserializer, de};
use csv::ReaderBuilder;
use serde_json::Value;
use crate::context::Version;

/** Provincial Claim Code
*
* Where:
*
*   TCAmtFloor: Total Claim Amount Floor
*
*   TCAmtCeil: Total Claim Amount Ceiling
*
*   TCP: Option 1 TCP
*
*   K1P: Option 1, K1P
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct ProvincialClaimCode {
    #[serde(deserialize_with="quoted_f64", rename="Total claim amount ($) from")]
    TCAmtFloor: Option<f64>,
    #[serde(deserialize_with="quoted_f64", rename="Total claim amount ($) to")]
    TCAmtCeil: Option<f64>,
    #[serde(deserialize_with="quoted_f64", rename="Option 1, TCP ($)")]
    TCP: Option<f64>,
    #[serde(deserialize_with="quoted_f64", rename="Option 1, K1P ($)")]
    K1P: Option<f64>,
}
fn quoted_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
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
        },
        Value::Number(n) => {
            n.as_f64()
        }
        _ => return Err(de::Error::custom("Wrong type, expected quoted f64.")),
    })
}

/** Provincial Claim Codes
*
* Where:
*   CC: Vec of Claim Codes (0..10)
*/
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ProvincialClaimCodes {
    pub CC: Vec<ProvincialClaimCode>,
}

impl ProvincialClaimCodes {

    /// Initialize Ontario Claim Codes
    pub fn init_on(version: &Version) -> Result<ProvincialClaimCodes, Box<dyn StdError>> {

        let mut records: Vec<ProvincialClaimCode> = Vec::new();

        let file_name = match version {
            Version::V2025_1 => {"cra-constants/v2025_1/cc-on-01-25e.csv"}
        };

        let mut rdr = ReaderBuilder::new().from_path(file_name)?;

        for result in rdr.deserialize() {
            let rec: ProvincialClaimCode = result?;
            records.push(rec.clone());
        }

        if records.len() != 11 {
            return Err("Datafile Corrupt. Expected 11 claim codes (0-10)".into());
        }

        Ok(Self {
            CC: records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_fed_claim_codes() {
        let result = ProvincialClaimCodes::init_on(&Version::V2025_1);
        assert!(result.is_ok());

        let fcc = result.unwrap();

        assert_eq!(fcc.CC.get(0).unwrap(), &ProvincialClaimCode{TCAmtFloor: None, TCAmtCeil: None, TCP: Some(0.0), K1P: Some(0.0)});
        assert_eq!(fcc.CC.get(5).unwrap(), &ProvincialClaimCode{TCAmtFloor: Some(20985.01), TCAmtCeil: Some(23731.0), TCP: Some(22358.0), K1P: Some(1129.08)});
        assert_eq!(fcc.CC.get(9).unwrap(), &ProvincialClaimCode{TCAmtFloor: Some(31969.01), TCAmtCeil: Some(34715.0), TCP: Some(33342.0), K1P: Some(1683.77)});
    }
}


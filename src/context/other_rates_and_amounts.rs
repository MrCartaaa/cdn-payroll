//! Other Rates and Amounts as defined by the CRA

use encoding_rs::UTF_8;
use std::collections::BTreeSet;
use std::fs::File;

use super::{Federal, Province, ProvinceKey, Version};
use csv::{ReaderBuilder, StringRecord};
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;

/// Other Federal and Outside of Canada Rates and Amounts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct OtherRatesAndAmounts {
    pub Federal: ORA,
    pub notCA: ORA,
}

pub trait ORAGetter {
    /// Initialize Provincial Other Rates and Amounts
    fn init_prov_otr(version: &Version, prov: &ProvinceKey) -> Result<ORA, Box<dyn StdError>> {
        let records = <Province as ORAGetter>::init_all(&version)?;

        Ok(match prov {
            ProvinceKey::AB => get_row_from_str(&records, "AB")?,
            ProvinceKey::BC => get_row_from_str(&records, "BC")?,
            ProvinceKey::MB => get_row_from_str(&records, "MB")?,
            ProvinceKey::NB => get_row_from_str(&records, "NB")?,
            ProvinceKey::NL => get_row_from_str(&records, "NL")?,
            ProvinceKey::NS => get_row_from_str(&records, "NS")?,
            ProvinceKey::NT => get_row_from_str(&records, "NT")?,
            ProvinceKey::NU => get_row_from_str(&records, "NU")?,
            ProvinceKey::ON => get_row_from_str(&records, "ON")?,
            ProvinceKey::PE => get_row_from_str(&records, "PE")?,
            ProvinceKey::QC => get_row_from_str(&records, "QC")?,
            ProvinceKey::SK => get_row_from_str(&records, "SK")?,
            ProvinceKey::YT => get_row_from_str(&records, "YT")?,
        })
    }

    /// Initialize Federal Other Rates and Amounts
    fn init_fed_otr(version: &Version) -> Result<OtherRatesAndAmounts, Box<dyn StdError>> {
        let recs = <Federal as ORAGetter>::init_all(version)?;
        Ok(OtherRatesAndAmounts {
            Federal: get_row_from_str(&recs, "Federal")?,
            notCA: get_row_from_str(&recs, "Outside Canada")?,
        })
    }

    /// Initialize all Other Rates and Amounts
    fn init_all(version: &Version) -> Result<Vec<ORA>, Box<dyn StdError>> {
        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/thrrtsmnts-01-25e.csv",
        };

        let file = File::open(file_name)?;
        let trscd = DecodeReaderBytesBuilder::new()
            .encoding(Some(UTF_8))
            .build(file);

        let mut rdr = ReaderBuilder::new()
            .flexible(true)
            .quoting(true)
            .from_reader(trscd);

        let headers = rdr.headers()?.clone();
        let mut new_headers = vec!["fed_prov"];

        for (i, header) in headers.iter().enumerate() {
            if i > 0 {
                new_headers.push(header);
            }
        }

        rdr.set_headers(StringRecord::from(new_headers));

        let mut records: Vec<ORA> = Vec::new();

        for result in rdr.deserialize() {
            let rec: ORA = result?;
            records.push(rec.clone());
        }

        if records.len() != 20 {
            return Err("Datafile corrrupt. Expected 20 rows.".into());
        }

        Ok(handle_multiaxis(records))
    }
}

fn get_row_from_str(recs: &Vec<ORA>, s: &str) -> Result<ORA, &'static str> {
    recs.iter()
        .filter(|otr| otr.fed_prov == s)
        .collect::<Vec<&ORA>>()
        .pop()
        .ok_or_else(|| "unable to locate ORA for {s}")
        .clone()
        .cloned()
}

fn handle_multiaxis(mut records: Vec<ORA>) -> Vec<ORA> {
    let mut last_fed_prov_row: usize = 9999;
    let mut last_t4atv1: Vec<Option<Vec<f64>>> = Vec::new();
    let mut last_v1rate: Vec<Option<Vec<f64>>> = Vec::new();
    let mut last_aatv2: Vec<Option<Vec<f64>>> = Vec::new();
    let mut last_v2rate: Vec<Option<Vec<f64>>> = Vec::new();
    let mut last_v2max: Vec<Option<Vec<f64>>> = Vec::new();
    let mut is_dirty: bool = false;

    for (i, rec) in records.clone().iter().enumerate() {
        if !rec.fed_prov.is_empty() && is_dirty {
            records[last_fed_prov_row] = ORA {
                fed_prov: records[last_fed_prov_row].fed_prov.clone(),
                BasicAmt: records[last_fed_prov_row].BasicAmt.clone(),
                IRate: records[last_fed_prov_row].IRate.clone(),
                LCPRate: records[last_fed_prov_row].LCPRate.clone(),
                LCPAmt: records[last_fed_prov_row].LCPAmt.clone(),
                CEA: records[last_fed_prov_row].CEA.clone(),
                S2: records[last_fed_prov_row].S2.clone(),
                T4atV1: handle_nested_option_f64(last_t4atv1.clone()),
                V1Rate: handle_nested_option_f64(last_v1rate.clone()),
                AatV2: handle_nested_option_f64(last_aatv2.clone()),
                V2Rate: handle_nested_option_f64(last_v2rate.clone()),
                V2Max: handle_nested_option_f64(last_v2max.clone()),
                YFactor: records[last_fed_prov_row].YFactor.clone(),
                Abat: records[last_fed_prov_row].Abat.clone(),
                Surtax: records[last_fed_prov_row].Surtax.clone(),
            };
            last_fed_prov_row = 9999;
            last_t4atv1 = Vec::new();
            last_v1rate = Vec::new();
            last_aatv2 = Vec::new();
            last_v2rate = Vec::new();
            last_v2max = Vec::new();
            is_dirty = false;
        }

        last_t4atv1.push(rec.T4atV1.clone());
        last_v1rate.push(rec.V1Rate.clone());
        last_aatv2.push(rec.AatV2.clone());
        last_v2rate.push(rec.V2Rate.clone());
        last_v2max.push(rec.V2Max.clone());

        if !rec.fed_prov.is_empty() {
            last_fed_prov_row = i;
        } else {
            is_dirty = true;
        }
    }

    records.retain(|i| !i.fed_prov.is_empty());
    records
}

fn handle_nested_option_f64(vecs: Vec<Option<Vec<f64>>>) -> Option<Vec<f64>> {
    let mut result: Vec<f64> = Vec::new();
    for v in vecs {
        if v.is_some() {
            result.append(&mut v.unwrap());
        }
    }
    if result.len() > 0 {
        return Some(result);
    }
    None
}

/** Other Rates and Amounts
*
* Where:
*
*   BasicAmt: Basic Amount (see enum BasicAmount)
*
*   IRate: Index Rate
*
*   LCPRate: LCP Rate
*
*   LCPAmt: LCP Amount
*
*   CEA: CEA
*
*   S2: S2
*
*   T4atV1: T4 to V1
*
*   V1Rate: V1 Rate
*
*   AatV2: A to V2
*
*   V2Rate: V2 Rate
*
*   V2Max: V2 Maximum
*
*   YFactor: factor used in Ontario [Y function](../provincial_income_tax/ontario/fn.Y.html)
*
*   Abat: Abatement
*
*   Surtax: Surtax
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct ORA {
    fed_prov: String,
    #[serde(deserialize_with = "basic_amt", rename = "Basic amount")]
    pub BasicAmt: Option<BasicAmount>,
    #[serde(deserialize_with = "quoted_f64", rename = "Index rate")]
    pub IRate: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "LCP rate")]
    pub LCPRate: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "LCP amount")]
    pub LCPAmt: Option<f64>,
    #[serde(deserialize_with = "quoted_f64")]
    pub CEA: Option<f64>,
    #[serde(deserialize_with = "quoted_f64")]
    pub S2: Option<f64>,
    #[serde(deserialize_with = "quoted_vec_f64", rename = "T4 to V1")]
    pub T4atV1: Option<Vec<f64>>,
    #[serde(deserialize_with = "quoted_vec_f64", rename = "V1 rate")]
    pub V1Rate: Option<Vec<f64>>,
    #[serde(deserialize_with = "quoted_vec_f64", rename = "A to V2")]
    pub AatV2: Option<Vec<f64>>,
    #[serde(deserialize_with = "quoted_vec_f64", rename = "V2 rate")]
    pub V2Rate: Option<Vec<f64>>,
    #[serde(deserialize_with = "quoted_vec_f64", rename = "V2 Maximum")]
    pub V2Max: Option<Vec<f64>>,
    #[serde(deserialize_with = "quoted_f64", rename = "Y factor")]
    pub YFactor: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "Abatement")]
    pub Abat: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "Surtax")]
    pub Surtax: Option<f64>,
}
fn quoted_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    Ok(match Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.trim().replace(",", "").replace("-", "");
            if v.is_empty() {
                return Ok(None);
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
fn quoted_vec_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<f64>>, D::Error> {
    #[allow(unreachable_code)]
    Ok(match Deserialize::deserialize(deserializer)? {
        Value::Number(n) => {
            let v = n.as_f64();
            if v.is_some() {
                return Ok(Some(vec![v.unwrap()]));
            };
            return Err(de::Error::custom("Wrong type, expected quoted f64."));
        }
        Value::String(s) => {
            let v = s.trim().replace(",", "").replace("-", "");
            if v.is_empty() {
                return Ok(None);
            };
            let val = v.parse::<f64>();
            if val.is_ok() {
                return Ok(Some(vec![val.unwrap()]));
            } else {
                return Ok(None);
            };
        }
        _ => return Err(de::Error::custom("Wrong type, expected quoted f64.")),
    })
}
fn basic_amt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<BasicAmount>, D::Error> {
    Ok(match Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.trim().replace(",", "").replace("-", "");
            if v.is_empty() {
                return Ok(None);
            }
            let val = v.parse::<f64>();
            if val.is_ok() {
                return Ok(Some(BasicAmount::BasicAmt(val.unwrap())));
            } else {
                match v.as_str() {
                    "BPAF" => Some(BasicAmount::Federal),
                    "BPANS" => Some(BasicAmount::NS),
                    "BPAMB" => Some(BasicAmount::MB),
                    "BPAYT" => Some(BasicAmount::YT),
                    _ => {
                        // there is a byte (below) that indicates None, but its difficult to find
                        // it (represented as <96> in view mode.) This tracks that only and
                        // provides the right enum type of None
                        let bytes_target = [239, 191, 189];
                        let bytes_str = v.clone().into_bytes();
                        let bytes_tree: BTreeSet<_> = bytes_str.iter().copied().collect();
                        if bytes_tree.iter().all(|i| bytes_target.contains(i)) {
                            return Ok(None);
                        }
                        return Err(de::Error::custom(format!(
                            "incorrect BPA string provided: {:?}.",
                            v
                        )));
                    }
                }
            }
        }
        Value::Number(n) => {
            let v = n.as_f64();
            if v.is_some() {
                return Ok(Some(BasicAmount::BasicAmt(v.unwrap())));
            }
            return Err(de::Error::custom("Wrong type, expected quoted f64."));
        }
        _ => return Err(de::Error::custom("Wrong type, expected quoted f64.")),
    })
}

/// Basic Amount
///
/// Can be either an f64 or an indicator of Basic Personal Amount (ie, BPAF, BPAMP)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub enum BasicAmount {
    BasicAmt(f64),
    Federal,
    MB,
    NS,
    YT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_other_rates_and_amounts() {
        struct ORA {}
        impl ORAGetter for ORA {}

        let result = ORA::init_all(&Version::V2025_1);
        assert!(&result.is_ok());
        let otr_recs = result.unwrap();
        assert_eq!(
            get_row_from_str(&otr_recs, "Federal").unwrap().BasicAmt,
            Some(BasicAmount::Federal)
        );
        assert!(get_row_from_str(&otr_recs, "ON")
            .unwrap()
            .T4atV1
            .unwrap()
            .contains(&5710.0));
        assert_eq!(get_row_from_str(&otr_recs, "QC").unwrap().LCPAmt, None);
        assert_eq!(get_row_from_str(&otr_recs, "AB").unwrap().IRate, Some(0.02));
        assert!(get_row_from_str(&otr_recs, "ON")
            .unwrap()
            .AatV2
            .unwrap()
            .contains(&200000.0));
        assert!(get_row_from_str(&otr_recs, "ON")
            .unwrap()
            .V2Rate
            .unwrap()
            .contains(&0.25));
        assert!(get_row_from_str(&otr_recs, "ON")
            .unwrap()
            .V2Max
            .unwrap()
            .contains(&450.0));
    }
}

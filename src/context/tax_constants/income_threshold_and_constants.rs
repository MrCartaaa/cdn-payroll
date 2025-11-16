//! Rates (R, V), Income Thresholds (A), and Constants (K, KP).

use super::{ProvinceKey, Version};
use csv::{ReaderBuilder, StringRecord};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::error::Error as StdError;

#[doc(hidden)]
pub trait ProvRITCGetter {
    /** Initialize Rates, Income Thresholds and Constants.
     */
    fn init_ritc(version: &Version, prov: &ProvinceKey) -> Result<ProvRITC, Box<dyn StdError>> {
        let records = init_all(&version)?;

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
}

#[doc(hidden)]
pub trait FedRITCGetter {
    fn init_ritc(version: &Version) -> Result<FedRITC, Box<dyn StdError>> {
        Ok(FedRITC::from_prov_ritc(get_row_from_str(
            &init_all(&version)?,
            "Federal",
        )?))
    }
}

fn init_all(version: &Version) -> Result<Vec<ProvRITC>, Box<dyn StdError>> {
    let file_name = match version {
        Version::V2025_1 => "cra-constants/v2025_1/rtsncmtrshldcnstnt-01-25e.csv",
    };

    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .quoting(true)
        .from_path(file_name)?;

    let headers = rdr.headers()?.clone();
    let mut new_headers = vec!["fed_prov", "key"];

    for (i, header) in headers.iter().enumerate() {
        if i > 1 {
            new_headers.push(header);
        }
    }

    rdr.set_headers(StringRecord::from(new_headers));

    let mut records: Vec<CSVData> = Vec::new();

    for rec in rdr.deserialize() {
        let result: CSVData = rec?;
        records.push(result);
    }

    if records.len() != 40 {
        return Err(format!(
            "Datafile corrupt. Expected 40 rows. got {} rows",
            records.len()
        )
        .into());
    }

    Ok(handle_multiaxis(records))
}

fn get_row_from_str(recs: &Vec<ProvRITC>, s: &str) -> Result<ProvRITC, &'static str> {
    recs.iter()
        .filter(|otr| otr.prov == s)
        .collect::<Vec<&ProvRITC>>()
        .pop()
        .ok_or_else(|| "unable to locate RITC for {s}")
        .clone()
        .cloned()
}

fn handle_multiaxis(records: Vec<CSVData>) -> Vec<ProvRITC> {
    let mut ma_recs: Vec<ProvRITC> = Vec::new();

    for rec in records.clone().iter() {
        let mut vals = handle_options(&rec);

        match rec {
            rec if rec.key == "A" => {
                let mut ritc = ProvRITC::new_empty(rec.fed_prov.clone());
                ritc.A.append(&mut vals);
                ma_recs.push(ritc);
            }
            rec if rec.key == "V" => {
                let rec = ma_recs.last_mut().unwrap();
                rec.V.append(&mut vals);
            }
            rec if rec.key == "K" || rec.key == "KP" => {
                let rec = ma_recs.last_mut().unwrap();
                rec.KP.append(&mut vals);
            }
            _ => {}
        };
    }
    ma_recs
}

fn handle_options(rec: &CSVData) -> Vec<f64> {
    let vec = vec![
        rec.first,
        rec.second,
        rec.third,
        rec.fourth,
        rec.fifth,
        rec.sixth,
        rec.seventh,
        rec.eighth,
    ];
    let f64_vec: Vec<f64> = vec.into_iter().filter_map(|x| x).collect();
    f64_vec
}

/** Provincial Income Threshold Constant, Rate and Bracket
*
* Where:
*
*   A: Annual taxable income Bracket
*
*   V: Provincial or territorial tax rate for the year
*
*   KP: Provincial or territorial constant
*/
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct ProvRITC {
    prov: String,
    pub A: Vec<f64>,
    pub V: Vec<f64>,
    pub KP: Vec<f64>,
}

impl ProvRITC {
    fn new_empty(prov: String) -> Self {
        Self {
            prov,
            A: Vec::new(),
            V: Vec::new(),
            KP: Vec::new(),
        }
    }
}

/** Federal Income Threshold Constant, Rate and Bracket
*
* Where:
*
*   A: Annual taxable income Bracket
*
*   R: Federal tax rate for the year
*
*   K: Federal Constant
*/
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct FedRITC {
    pub A: Vec<f64>,
    pub R: Vec<f64>,
    pub K: Vec<f64>,
}

#[doc(hidden)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct FederalThreshold {
    pub A: f64,
    pub R: f64,
    pub K: f64,
}

impl FedRITC {
    fn from_prov_ritc(prov_ritc: ProvRITC) -> Self {
        Self {
            A: prov_ritc.A,
            R: prov_ritc.V,
            K: prov_ritc.KP,
        }
    }

    #[allow(non_snake_case)]
    pub fn get_income_threshold(&self, A: f64) -> Result<FederalThreshold, anyhow::Error> {
        let max_threshold = self.A.len();
        for (i, a) in self.A.iter().enumerate() {
            if (i + 1) > max_threshold {
                let n = self.A[i + 1];
                if A > n && A < *a {
                    return Ok(FederalThreshold {
                        A: a.to_owned(),
                        R: self.R[i],
                        K: self.K[i],
                    });
                }
            }
            return Ok(FederalThreshold {
                A: a.to_owned(),
                R: self.R[i],
                K: self.K[i],
            });
        }
        Err(anyhow::anyhow!("unable to find federal income threshold."))
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct CSVData {
    fed_prov: String,
    key: String,
    #[serde(deserialize_with = "quoted_f64", rename = "1st")]
    first: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "2nd")]
    second: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "3rd")]
    third: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "4th")]
    fourth: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "5th")]
    fifth: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "6th")]
    sixth: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "7th")]
    seventh: Option<f64>,
    #[serde(deserialize_with = "quoted_f64", rename = "8th")]
    eighth: Option<f64>,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_itc() {
        let result = init_all(&Version::V2025_1);
        assert!(result.is_ok());
        let itc = result.unwrap();

        assert_eq!(itc[0].A[1], 57375.0);
        assert_eq!(itc[9].V[1], 0.0915);
        assert_eq!(itc[7].KP.get(7), None);
    }
}

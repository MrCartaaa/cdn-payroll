//! Rates (R, V), Income Thresholds (A), and Constants (K, KP).

use super::Version;
use csv::{ReaderBuilder, StringRecord, StringRecordsIter};
use std::error::Error;

/// Federal and Provincial Rates (R, V), Income Thresholds (A), and Constants(K, P)
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct IncomeThresholdAndConstants {
    pub Federal: ITFedConst,
    pub AB: ProvITCRB,
    pub BC: ProvITCRB,
    pub MB: ProvITCRB,
    pub NB: ProvITCRB,
    pub NL: ProvITCRB,
    pub NS: ProvITCRB,
    pub NT: ProvITCRB,
    pub NU: ProvITCRB,
    pub ON: ProvITCRB,
    pub PE: ProvITCRB,
    pub SK: ProvITCRB,
    pub YT: ProvITCRB,
}

impl IncomeThresholdAndConstants {
    /** Initialize Rates, Income Thresholds and Constants.
     */
    pub fn init(version: &Version) -> Result<IncomeThresholdAndConstants, Box<dyn Error>> {
        let file_name = match version {
            Version::V2025_1 => "cra-constants/v2025_1/rtsncmtrshldcnstnt-01-25e.csv",
        };

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .quoting(true)
            .from_path(file_name)?;
        let mut iter = rdr.records();

        // not using headers, skip;
        iter.next();

        let itcf = Self::get_itcf_item(&mut iter);
        let itc_ab = Self::get_itcp_item(&mut iter);
        let itc_bc = Self::get_itcp_item(&mut iter);
        let itc_mb = Self::get_itcp_item(&mut iter);
        let itc_nb = Self::get_itcp_item(&mut iter);
        let itc_nl = Self::get_itcp_item(&mut iter);
        let itc_ns = Self::get_itcp_item(&mut iter);
        let itc_nt = Self::get_itcp_item(&mut iter);
        let itc_nu = Self::get_itcp_item(&mut iter);
        let itc_on = Self::get_itcp_item(&mut iter);
        let itc_pe = Self::get_itcp_item(&mut iter);
        let itc_sk = Self::get_itcp_item(&mut iter);
        let itc_yt = Self::get_itcp_item(&mut iter);

        Ok(Self {
            Federal: itcf,
            AB: itc_ab,
            BC: itc_bc,
            MB: itc_mb,
            NB: itc_nb,
            NL: itc_nl,
            NS: itc_ns,
            NT: itc_nt,
            NU: itc_nu,
            ON: itc_on,
            PE: itc_pe,
            SK: itc_sk,
            YT: itc_yt,
        })
    }

    fn get_itcf_item<R>(iter: &mut StringRecordsIter<R>) -> ITFedConst
    where
        R: std::io::Read,
    {
        ITFedConst::new(
            Self::get_itc_row(iter.next().unwrap().unwrap()),
            Self::get_itc_row(iter.next().unwrap().unwrap()),
            Self::get_itc_row(iter.next().unwrap().unwrap()),
        )
    }

    fn get_itcp_item<R>(iter: &mut StringRecordsIter<R>) -> ProvITCRB
    where
        R: std::io::Read,
    {
        ProvITCRB::new(
            Self::get_itc_row(iter.next().unwrap().unwrap()),
            Self::get_itc_row(iter.next().unwrap().unwrap()),
            Self::get_itc_row(iter.next().unwrap().unwrap()),
        )
    }

    fn get_itc_row(rec: StringRecord) -> ITBracket {
        ITBracket::new(
            Self::optionf64(rec.get(2)),
            Self::optionf64(rec.get(3)),
            Self::optionf64(rec.get(4)),
            Self::optionf64(rec.get(5)),
            Self::optionf64(rec.get(6)),
            Self::optionf64(rec.get(7)),
            Self::optionf64(rec.get(8)),
            Self::optionf64(rec.get(9)),
        )
    }

    fn optionf64(col: Option<&str>) -> Option<f64> {
        match col {
            Some(col) => {
                let v = col.trim().replace(",", "");
                if v.is_empty() {
                    return None;
                } else {
                    return Some(v.parse::<f64>().unwrap());
                }
            }
            None => None,
        }
    }
}

/// Income Threshold Brackets
#[derive(Debug)]
pub struct ITBracket {
    pub first: Option<f64>,
    pub second: Option<f64>,
    pub third: Option<f64>,
    pub fourth: Option<f64>,
    pub fifth: Option<f64>,
    pub sixth: Option<f64>,
    pub seventh: Option<f64>,
    pub eighth: Option<f64>,
}

impl ITBracket {
    fn new(
        first: Option<f64>,
        second: Option<f64>,
        third: Option<f64>,
        fourth: Option<f64>,
        fifth: Option<f64>,
        sixth: Option<f64>,
        seventh: Option<f64>,
        eighth: Option<f64>,
    ) -> Self {
        Self {
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
            seventh,
            eighth,
        }
    }
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
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ProvITCRB {
    pub A: ITBracket,
    pub V: ITBracket,
    pub KP: ITBracket,
}

impl ProvITCRB {
    #[allow(non_snake_case)]
    fn new(A: ITBracket, V: ITBracket, KP: ITBracket) -> Self {
        Self { A, V, KP }
    }
}

/** Federal Income Threshold Constant, Rate and Bracket
*
* Where:
*
*   A: Annual taxable income Bracket
*
*  V: Federal tax rate for the year
*
*   K: Federal Constant
*/
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ITFedConst {
    pub A: ITBracket,
    pub V: ITBracket,
    pub K: ITBracket,
}

impl ITFedConst {
    #[allow(non_snake_case)]
    fn new(A: ITBracket, V: ITBracket, K: ITBracket) -> Self {
        Self { A, V, K }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_itc() {
        let result = IncomeThresholdAndConstants::init(&Version::V2025_1);
        assert!(result.is_ok());
        let itc = result.unwrap();

        assert_eq!(itc.Federal.A.second, Some(57375.0));
        assert_eq!(itc.ON.V.second, Some(0.0915));
        assert_eq!(itc.NT.KP.eighth, None);
    }
}

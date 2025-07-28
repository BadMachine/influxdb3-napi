use reqwest::Url;
use crate::client::options::{Precision, TimeUnitV2, TimeUnitV3, WriteOptions};

pub mod point;
mod point_values;

pub fn get_write_path(database: String, org: Option<String>, _write_options: Option<crate::client::options::WriteOptions>) -> (Url, WriteOptions) {
    let write_options = _write_options.unwrap_or_default();
    let mut query_params: Vec<(String, String)> = Vec::new();

    let write_path = match write_options.no_sync {
        Some(true) => WRITE_V3_PATH,
        _ => WRITE_V2_PATH,
    };

    let precision = match write_options.precision {
        Some(precision) => {
            match precision {
                Precision::V3(v3_precision) => {
                    if write_options.no_sync.is_some() { v3_precision.to_string() } else {
                        match v3_precision {
                            TimeUnitV3::Second => { TimeUnitV2::Second.to_string() },
                            TimeUnitV3::Millisecond => { TimeUnitV2::Millisecond.to_string() },
                            TimeUnitV3::Microsecond => { TimeUnitV2::Microsecond.to_string() },
                            TimeUnitV3::Nanosecond => { TimeUnitV2::Nanosecond.to_string() },
                        }
                    }
                }
                Precision::V2(v2_precision) => {
                    if write_options.no_sync.is_none() { v2_precision.to_string() } else {
                        match v2_precision {
                            TimeUnitV2::Second => TimeUnitV3::Second.to_string(),
                            TimeUnitV2::Millisecond => TimeUnitV3::Millisecond.to_string(),
                            TimeUnitV2::Microsecond => TimeUnitV3::Microsecond.to_string(),
                            TimeUnitV2::Nanosecond => TimeUnitV3::Nanosecond.to_string()
                        }
                    }
                }
            }
        }
        _ => TimeUnitV3::Nanosecond.to_string()
    };

    query_params.push((String::from("db"), database));
    query_params.push((String::from(PRECISION_QUERY_NAME), precision));

    match org {
        Some(org) => query_params.push((String::from("org"), org)),
        _ => {  }
    }


    (reqwest::Url::parse_with_params(write_path, &query_params).unwrap(), write_options)
}

static WRITE_V3_PATH: &'static str = "/api/v3/write_lp";
static WRITE_V2_PATH: &'static str = "/api/v2/write";
static PRECISION_QUERY_NAME: &'static str = "precision";
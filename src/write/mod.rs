use crate::client::options::{Precision, TimeUnitV2, TimeUnitV3, WriteOptions};
use reqwest::Url;

pub fn get_write_path(
  url: &str,
  database: String,
  org: Option<String>,
  _write_options: Option<WriteOptions>,
) -> napi::Result<(Url, WriteOptions)> {
  let write_options = _write_options.unwrap_or_default();
  let mut query_params: Vec<(String, String)> = Vec::new();

  let write_path = if write_options.no_sync.unwrap_or(false) {
    WRITE_V3_PATH
  } else {
    WRITE_V2_PATH
  };

  let final_url = format!("{url}{write_path}");

  let precision = match write_options.precision {
    Some(precision) => match precision {
      Precision::V3(v3_precision) => {
        if write_options.no_sync.is_some() {
          v3_precision.to_string()
        } else {
          match v3_precision {
            TimeUnitV3::Second => TimeUnitV3::Second.to_string(),
            TimeUnitV3::Millisecond => TimeUnitV3::Millisecond.to_string(),
            TimeUnitV3::Microsecond => TimeUnitV3::Microsecond.to_string(),
            TimeUnitV3::Nanosecond => TimeUnitV3::Nanosecond.to_string(),
          }
        }
      }
      Precision::V2(v2_precision) => {
        if write_options.no_sync.is_none() {
          v2_precision.to_string()
        } else {
          match v2_precision {
            TimeUnitV2::Second => TimeUnitV2::Second.to_string(),
            TimeUnitV2::Millisecond => TimeUnitV2::Millisecond.to_string(),
            TimeUnitV2::Microsecond => TimeUnitV2::Microsecond.to_string(),
            TimeUnitV2::Nanosecond => TimeUnitV2::Nanosecond.to_string(),
          }
        }
      }
    },
    _ => TimeUnitV3::Nanosecond.to_string(),
  };

  match write_options.no_sync {
    Some(true) => {
      query_params.push((String::from("db"), database));
    }
    _ => {
      query_params.push((String::from("bucket"), database));
    }
  }

  query_params.push((String::from(PRECISION_QUERY_NAME), precision));

  if let Some(org) = org {
    query_params.push((String::from("org"), org))
  }

  let url = Url::parse_with_params(final_url.as_str(), &query_params);

  match url {
    Ok(url) => Ok((url, write_options)),
    Err(e) => {
      println!("Error occurred in get_write_path: {e:?}");
      Err(napi::Error::from_reason("Error parsing URL"))
    }
  }
}

static WRITE_V3_PATH: &str = "/api/v3/write_lp";
static WRITE_V2_PATH: &str = "/api/v2/write";
static PRECISION_QUERY_NAME: &str = "precision";

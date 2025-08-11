use napi_derive::napi;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[napi_derive::napi(string_enum)]
pub enum QueryType {
  #[napi(value = "sql")]
  Sql,
  #[napi(value = "influxql")]
  InfluxQl,
  #[napi(value = "flight_sql")]
  FlightSql,
}

impl QueryType {
  pub fn str(&self) -> &'static str {
    match self {
      Self::Sql => "sql",
      Self::InfluxQl => "influxql",
      Self::FlightSql => "flightsql",
    }
  }
}

impl Display for QueryType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let str = match self {
      QueryType::Sql => "sql",
      QueryType::InfluxQl => "influxql",
      QueryType::FlightSql => "flightsql",
    };
    write!(f, "{str}")
  }
}

#[napi]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Precision {
  V2(TimeUnitV2),
  V3(TimeUnitV3),
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnitV2 {
  /// Time in seconds.
  #[napi(value = "s")]
  Second,
  /// Time in milliseconds.
  #[napi(value = "ms")]
  Millisecond,
  /// Time in microseconds.
  #[napi(value = "us")]
  Microsecond,
  /// Time in nanoseconds.
  #[napi(value = "ns")]
  Nanosecond,
}

impl fmt::Display for TimeUnitV2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TimeUnitV2::Second => write!(f, "s"),
      TimeUnitV2::Millisecond => write!(f, "ms"),
      TimeUnitV2::Microsecond => write!(f, "us"),
      TimeUnitV2::Nanosecond => write!(f, "ns"),
    }
  }
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnitV3 {
  /// Time in seconds.
  #[napi(value = "second")]
  Second,
  /// Time in milliseconds.
  #[napi(value = "millisecond")]
  Millisecond,
  /// Time in microseconds.
  #[napi(value = "microsecond")]
  Microsecond,
  /// Time in nanoseconds.
  #[napi(value = "nanosecond")]
  Nanosecond,
}

impl fmt::Display for TimeUnitV3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TimeUnitV3::Second => write!(f, "second"),
      TimeUnitV3::Millisecond => write!(f, "millisecond"),
      TimeUnitV3::Microsecond => write!(f, "microsecond"),
      TimeUnitV3::Nanosecond => write!(f, "nanosecond"),
    }
  }
}

#[cfg_attr(not(feature = "native"), napi(object))]
pub struct WriteOptions {
  /** Precision to use in writes for timestamp. default ns */
  pub precision: Option<Precision>,
  /** HTTP headers that will be sent with every write request */
  //headers?: {[key: string]: string}
  pub headers: Option<HashMap<String, String>>,
  /** When specified, write bodies larger than the threshold are gzipped  */
  pub gzip: bool,
  /**
   * Instructs the server whether to wait with the response until WAL persistence completes.
   * noSync=true means faster write but without the confirmation that the data was persisted.
   *
   * Note: This option is supported by InfluxDB 3 Core and Enterprise servers only.
   * For other InfluxDB 3 server types (InfluxDB Clustered, InfluxDB Clould Serverless/Dedicated)
   * the write operation will fail with an error.
   *
   * Default value: false.
   */
  pub no_sync: Option<bool>,

  pub default_tags: Option<HashMap<String, String>>,
}

impl Default for WriteOptions {
  fn default() -> Self {
    Self {
      precision: Some(Precision::V3(TimeUnitV3::Nanosecond)),
      headers: None,
      gzip: true,
      no_sync: Some(true),
      default_tags: None,
    }
  }
}

pub fn to_header_map(
  map: &HashMap<String, String>,
) -> Result<HeaderMap, Box<dyn std::error::Error + Send + Sync>> {
  let mut headers = HeaderMap::new();

  for (k, v) in map {
    let name = HeaderName::from_bytes(k.as_bytes())?;
    let value = HeaderValue::from_str(v)?;
    headers.insert(name, value);
  }

  Ok(headers)
}

// #[derive(Default)]
// #[cfg_attr(not(feature = "native"), napi_derive::napi(object))]
// pub struct ClientOptions {
//   pub write_options: Option<WriteOptions>,
//   pub flight_options: Option<FlightOptions>,
// }

#[cfg_attr(not(feature = "native"), napi_derive::napi(object))]
#[derive(Clone)]
pub struct FlightOptions {
  pub keep_alive_interval: Option<u32>,
  pub keep_alive_timeout: Option<u32>,
}

impl Default for FlightOptions {
  fn default() -> Self {
    Self {
      keep_alive_interval: Some(5),
      keep_alive_timeout: Some(20),
    }
  }
}

#[cfg_attr(not(feature = "native"), napi_derive::napi(object))]
pub struct QueryPayload {
  pub database: String,
  pub query: String,
  pub _type: Option<QueryType>,
  pub params: Option<HashMap<String, String>>,
}

impl Into<String> for QueryPayload {
  fn into(self) -> String {
    let json = match self.params {
      Some(params) => json!({
          "database": self.database,
          "sql_query": self.query,
          "query_type": self._type.unwrap_or(QueryType::Sql),
          "params": params
      }),
      None => json!({
          "database": self.database,
          "sql_query": self.query,
          "query_type": self._type.unwrap_or(QueryType::Sql),
      }),
    };

    json.to_string()
  }
}

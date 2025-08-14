use serde::{Deserialize, Serialize};

pub(crate) mod client;

#[derive(Deserialize, Serialize)]
pub struct Series {
  name: String,
  pub(crate) columns: Vec<String>,
  pub(crate) values: Vec<Vec<serde_json::Value>>,
}

#[derive(Deserialize, Serialize)]
pub struct QueryDataV1 {
  statement_id: u32,
  pub(crate) series: Vec<Series>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct HttpQueryResponseV1 {
  pub(crate) results: Vec<QueryDataV1>,
}

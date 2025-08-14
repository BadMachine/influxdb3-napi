use crate::client::browser::HttpQueryResponseV1;
use crate::client::http_client::get_http_client;
use crate::client::options::{FlightOptions, QueryPayload, WriteOptions};
use crate::query::browser::query_processor::into_stream;
use crate::serializer::browser::Serializer;
use napi::bindgen_prelude::*;
use napi::bindgen_prelude::{Buffer, Either, ReadableStream};
use napi::tokio_stream::wrappers::ReceiverStream;
use napi::Env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[napi_derive::napi]
pub struct InfluxDBClient {
  addr: String,
  http_client: Client,
  serializer: Serializer,
  options: FlightOptions,
}

// replace it with #[napi_derive::napi] in the future
// impl InfluxClientTrait for InfluxDBClient {
#[napi_derive::napi]
impl InfluxDBClient {
  #[napi(constructor)]
  pub fn new(
    addr: String,
    token: Option<String>,
    serializer: Option<Serializer>,
    options: Option<FlightOptions>,
  ) -> Self {
    Self {
      addr,
      http_client: get_http_client(token.unwrap_or(String::from(""))),
      serializer: serializer.unwrap_or_default(),
      options: options.unwrap_or_default(),
    }
  }

  #[napi_derive::napi]
  pub fn query(
    &mut self,
    query_payload: QueryPayload,
    env: &Env,
  ) -> napi::Result<Either<ReadableStream<'_, Map<String, Value>>, ReadableStream<'_, Buffer>>> {
    let stream = self.query_inner(query_payload, env)?;
    Ok(Either::A(stream))
  }

  pub fn query_inner(
    &mut self,
    query_payload: QueryPayload,
    env: &Env,
  ) -> Result<ReadableStream<'_, serde_json::Map<String, serde_json::Value>>> {
    use napi::bindgen_prelude::block_on;

    let stream: ReceiverStream<Result<serde_json::Map<String, serde_json::Value>>> =
      block_on(async {
        let url = format!("{}/query", self.addr);

        let response = self
          .http_client
          .get(&url)
          .query(&[
            ("db", query_payload.database.clone()),
            ("q", query_payload.query.clone()),
          ])
          .send()
          .await
          .map_err(|e| Error::from_reason(format!("HTTP request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
          return Err(Error::from_reason(format!(
            "InfluxDB returned non-success status: {}",
            status
          )));
        }

        let data: HttpQueryResponseV1 = response
          .json()
          .await
          .map_err(|e| Error::from_reason(format!("Failed to parse JSON: {}", e)))?;

        Ok(into_stream(data))
      })?;

    ReadableStream::new(env, stream)
  }

  #[napi_derive::napi]
  pub fn write(
    &mut self,
    lines: Vec<String>,
    database: String,
    write_options: Option<WriteOptions>,
    org: Option<String>,
  ) -> Result<()> {
    todo!()
  }
}

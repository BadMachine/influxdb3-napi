use crate::client::http_client::get_http_client;
use crate::client::options::{FlightOptions, QueryPayload, WriteOptions};
use crate::serializer::browser::Serializer;
use futures_util::StreamExt;
use napi::bindgen_prelude::*;
use reqwest::Client;
use tokio::task::LocalSet;

#[napi_derive::napi]
pub struct InfluxDBClient {
  addr: String,
  http_client: Client,
  serializer: Serializer,
  options: FlightOptions,
}

#[napi_derive::napi]
impl InfluxDBClient {
  #[napi_derive::napi(constructor)]
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
  pub async unsafe fn query(
    &mut self,
    query_payload: QueryPayload,
    // env: &Env,
    // ) -> napi::Result<Either<ReadableStream<'_, Map<String, Value>>, ReadableStream<'_, Buffer>>> {
  ) -> napi::Result<u32> {
    // let stream = self
    //   .query_inner(
    //     query_payload,
    //     // env
    //   )
    //   .await;
    // Ok(Either::A(stream))
    println!("query payload: {:?}", query_payload.query);

    unimplemented!();

    let stream = 1u32;
    Ok(stream)
  }

  pub async unsafe fn query_inner(
    &mut self,
    query_payload: QueryPayload,
    // env: &Env,
  ) -> napi::Result<u32> {
    // ) -> Result<ReadableStream<'_, serde_json::Map<String, serde_json::Value>>> {
    println!("1");

    println!("Print inside");

    // let url = format!("{}/query", self.addr);
    //
    // let response = &self
    //   .http_client
    //   .get(&url)
    //   .query(&[
    //     ("db", query_payload.database.clone()),
    //     ("q", query_payload.query.clone()),
    //   ])
    //   .send()
    //   .await
    //   .map_err(|e| Error::from_reason(format!("HTTP request failed: {}", e)))?;

    let client = Client::new();
    let response = client
      .get("https://google.com")
      .send()
      .await
      .map_err(|e| Error::from_reason(format!("HTTP request failed: {}", e)))?;

    let status = response.status();
    println!("Status: {:?}", status);

    if !status.is_success() {
      return Err(Error::from_reason(format!(
        "InfluxDB returned non-success status: {}",
        status
      )));
    }

    println!("tratatatatata result: {}", 1);

    unimplemented!();
    // let stream: ReceiverStream<Result<serde_json::Map<String, serde_json::Value>>> =
    //   block_on(async {
    //     let url = format!("{}/query", self.addr);
    //
    //     let mut response = self
    //       .http_client
    //       .get(&url)
    //       .query(&[
    //         ("db", query_payload.database.clone()),
    //         ("q", query_payload.query.clone()),
    //       ])
    //       .send()
    //       .await
    //       .map_err(|e| Error::from_reason(format!("HTTP request failed: {}", e)))?;
    //
    //     // while let Some(chunk) = response.chunk().await? {
    //     //   println!("Chunk: {chunk:?}");
    //     // }
    //     // .map_err(|e| Error::from_reason(format!("HTTP request failed: {}", e)))?;
    //
    //     let status = response.status();
    //     if !status.is_success() {
    //       return Err(Error::from_reason(format!(
    //         "InfluxDB returned non-success status: {}",
    //         status
    //       )));
    //     }
    //
    //     let data: HttpQueryResponseV1 = response
    //       .json()
    //       .await
    //       .map_err(|e| Error::from_reason(format!("Failed to parse JSON: {}", e)))?;
    //
    //     Ok(into_stream(data))
    //   })?;
    //
    // ReadableStream::new(env, stream)
  }

  // #[napi_derive::napi]
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

#[napi_derive::napi]
pub async fn test() {
  println!("OUTSIDE OF ASYNC RUNTIME --------------------");

  // tokio::task::spawn_local(async {
  //   let res = reqwest::get("http://www.rust-lang.org").await.unwrap();
  //
  //   println!("--- Response ---- {:?}", res.status());
  // });

  wasm_bindgen_futures::spawn_local(async move {
    match reqwest::get("http://www.rust-lang.org").await {
      Ok(resp) => {
        println!("123");
      }
      Err(err) => {
        println!("Error: {:?}", err);
      }
    }
  });
}

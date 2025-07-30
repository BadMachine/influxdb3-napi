pub(crate) mod options;

use crate::query::by_batch::QueryResultByBatch;

use std::fmt;
use arrow::ipc::Precision;
use crate::serializer::Serializer;
use arrow_flight::{FlightClient, Ticket};
use reqwest::header::HeaderMap;
use reqwest::Client;
use tonic::codegen::Bytes;
use tonic::transport::Channel;
use crate::client::options::{to_header_map, ClientOptions, WriteOptions};
use crate::write::get_write_path;
// use tonic_web_wasm_client::Client;

#[napi_derive::napi(string_enum)]
pub enum QueryType {
  #[napi(value = "sql")]
  Sql,
  #[napi(value = "influxql")]
  Influxql,
}

impl fmt::Display for QueryType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      QueryType::Sql => write!(f, "sql"),
      QueryType::Influxql => write!(f, "influxql"),
    }
  }
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
pub struct InfluxDBClient {
  flight_client: FlightClient,
  serializer: Option<Serializer>,
  http_client: Client,
  // options: ClientOptions,
}


#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl InfluxDBClient {
  #[cfg_attr(not(feature = "native"), napi_derive::napi(constructor))]
  pub fn new(addr: String, token: Option<String>, serializer: Option<Serializer>, options: Option<ClientOptions>) -> Self {
    #[cfg(not(feature = "native"))]
    use napi::bindgen_prelude::block_on;
    #[cfg(not(feature = "native"))]
    let channel = block_on(async {
      Channel::from_shared(addr)
        .unwrap()
        .connect()
        .await
        .expect("error connecting")
    });
    #[cfg(feature = "native")]
    let channel = Channel::from_shared(addr).unwrap().connect_lazy();
    let http_client = get_http_client(token.clone().unwrap_or(String::from("")));

    let mut flight_client = FlightClient::new(channel);
    if let Some(token) = token {
      flight_client
        .add_header("authorization", format!("Bearer {}", token).as_str())
        .unwrap();
    }

    Self {
      flight_client,
      serializer,
      http_client,
      // options: options.unwrap_or_default()
    }
  }

  #[cfg(not(feature = "native"))]
  #[cfg_attr(not(feature = "native"), napi_derive::napi)]
  /// # Safety
  ///
  /// This function should not be called before the horsemen are ready.
  pub async unsafe fn query_batch(
    &mut self,
    database: String,
    query: String,
    _type: QueryType,
  ) -> Result<QueryResultByBatch, napi::Error> {
    let payload = format!(
      "{{ \"database\": \"{}\", \"sql_query\": \"{}\", \"query_type\": \"{}\" }}",
      database, query, _type
    )
    .replace("\n", " ");

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    let response = self.flight_client.do_get(ticket).await.unwrap();

    let result = QueryResultByBatch::new(response, self.serializer.clone());

    Ok(result)
  }

  #[cfg(feature = "native")]
  pub async fn query_batch(
    &mut self,
    database: String,
    query: String,
    _type: QueryType,
  ) -> Result<QueryResultByBatch, napi::Error> {
    let payload = format!(
      "{{ \"database\": \"{}\", \"sql_query\": \"{}\", \"query_type\": \"{}\" }}",
      database, query, _type
    )
    .replace("\n", " ");

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    let response = self.flight_client.do_get(ticket).await.unwrap();

    let result = QueryResultByBatch::new(response, self.serializer.clone());

    Ok(result)
  }

  #[cfg_attr(not(feature = "native"), napi_derive::napi)]
  pub async unsafe fn write(&mut self, lines: Vec<String>, database: String, write_options: Option<WriteOptions>, org: Option<String>) {
    let (url, write_options) = get_write_path(database, org, write_options);
    let headers = to_header_map(&write_options.headers.unwrap_or_default()).unwrap();
    let response = self.http_client
    .post(url)
    .body(lines.join("\n"))
    .headers(headers)
    .send()
    .await;
  }
}

pub fn get_http_client(token: String) -> Client {
  let mut headers = HeaderMap::with_capacity(2);
  headers.insert(
    reqwest::header::AUTHORIZATION,
    reqwest::header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).expect("REASON"),
  );
  headers.insert(
    reqwest::header::CONTENT_TYPE,
    reqwest::header::HeaderValue::from_static("text/plain; charset=utf-8"),
  );
  Client::builder().default_headers(headers).build().unwrap()
}

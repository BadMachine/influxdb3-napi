use crate::query::by_batch::QueryResultByBatch;

use std::fmt;

use crate::serializer::Serializer;
use arrow_flight::{Action, FlightClient, Ticket};
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder};
use tokio_stream::StreamExt;
use tonic::codegen::Bytes;
use tonic::transport::Channel;
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
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl InfluxDBClient {
  #[cfg_attr(not(feature = "native"), napi_derive::napi(constructor))]
  pub fn new(addr: String, token: Option<String>, serializer: Option<Serializer>) -> Self {
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

  #[cfg(feature = "native")]
  pub async fn write(&mut self) {
    let payload = format!("temperature,location=north value=60.0");

    self
      .flight_client
      .add_header("database", "birds")
      .expect("TODO: panic message");

    let request = Action::new("my_action", payload);

    let mut response = self.flight_client.do_action(request).await.unwrap();

    while let Some(a) = response.next().await {
      println!("{:?}", a);
    }

    let descriptor = arrow_flight::FlightDescriptor::new_path(vec![String::from("123")]);
    //
    // let a = self.flight_client.do_put(descriptor);

    // self.flight_client.add_header("database", "birds").expect("TODO: panic message");
    //
    // let mut response = self.flight_client.do_put(request).await.unwrap();
    //
    // while let Some(a) = response.next().await {
    //   println!("{:?}", a);
    // }

    // let result = QueryResultByBatch::new(response, self.serializer.clone());

    // Ok(result)
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

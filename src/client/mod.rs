pub mod options;

use crate::client::options::QueryType;
pub use crate::client::options::{to_header_map, ClientOptions, WriteOptions};
use crate::query::query_processor::QueryProcessor;
use crate::serializer::library_serializer::LibrarySerializer;
use crate::serializer::raw_serializer::RawSerializer;
use crate::serializer::unsafe_serializer::UnsafeSerializer;
use crate::serializer::Serializer;
use crate::write::get_write_path;
use crate::{Status, Value};
use arrow_flight::{FlightClient, Ticket};
use napi::bindgen_prelude::{Either3, ReadableStream};
// use napi::tokio_stream::{StreamExt};
use napi::Env;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
// use tokio_util::io::{StreamReader};
use tonic::codegen::Bytes;
use tonic::transport::{Channel, Endpoint};

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
pub struct InfluxDBClient {
  addr: String,
  flight_client: FlightClient,
  serializer: Serializer,
  http_client: Client,
  // options: ClientOptions,
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl InfluxDBClient {
  #[cfg_attr(not(feature = "native"), napi_derive::napi(constructor))]
  pub fn new(
    addr: String,
    token: Option<String>,
    serializer: Option<Serializer>,
    options: Option<ClientOptions>,
  ) -> Self {
    #[cfg(not(feature = "native"))]
    use napi::bindgen_prelude::block_on;
    #[cfg(not(feature = "native"))]
    let channel = block_on(async {
      get_channel(addr.clone())
        .connect()
        .await
        .expect("error connecting")
    });

    #[cfg(feature = "native")]
    let channel = get_channel(addr.clone()).connect_lazy();

    let http_client = get_http_client(token.clone().unwrap_or(String::from("")));

    let mut flight_client = FlightClient::new(channel);

    if let Some(token) = token {
      flight_client
        .add_header("authorization", format!("Bearer {token}").as_str())
        .unwrap();
    }

    Self {
      addr,
      flight_client,
      serializer: serializer.unwrap_or(Serializer::Unsafe),
      http_client,
      // options: options.unwrap_or_default()
    }
  }

  pub async fn query_inner(
    &mut self,
    database: String,
    query: String,
    _type: Option<QueryType>,
    env: &Env,
  ) -> napi::Result<
    Either3<
      ReadableStream<'_, serde_json::Value>,
      ReadableStream<'_, HashMap<String, Option<Value>>>,
      ReadableStream<'_, Vec<u8>>,
    >,
  > {
    //ReadableStream<Vec<Either3<Value, serde_json::Value, u8>>>
    let payload = format!(
      "{{ \"database\": \"{}\", \"sql_query\": \"{}\", \"query_type\": \"{}\" }}",
      database,
      query,
      _type.unwrap_or(QueryType::Sql)
    )
    .replace("\n", " ");

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    let response = self.flight_client.do_get(ticket).await.unwrap();

    match self.serializer.clone() {
      Serializer::Unsafe => {
        let processor = QueryProcessor::<UnsafeSerializer>::new(response, Serializer::Unsafe);
        let stream = processor.into_stream(env)?;
        Ok(Either3::A(stream))
      }
      Serializer::Library => {
        let processor = QueryProcessor::<LibrarySerializer>::new(response, Serializer::Library);
        let stream = processor.into_stream(env)?;
        Ok(Either3::B(stream))
      }
      Serializer::Raw => {
        let processor = QueryProcessor::<RawSerializer>::new(response, Serializer::Raw);
        let stream = processor.into_stream(env)?;
        Ok(Either3::C(stream))
      }
    }
  }

  #[cfg(not(feature = "native"))]
  #[napi_derive::napi]
  /// # Safety
  ///
  /// This function should not be called before the horsemen are ready.
  pub async unsafe fn query(
    &mut self,
    database: String,
    query: String,
    _type: Option<QueryType>,
    env: &Env,
  ) -> napi::Result<
    Either3<
      ReadableStream<'_, serde_json::Value>,
      ReadableStream<'_, HashMap<String, Option<Value>>>,
      ReadableStream<'_, Vec<u8>>,
    >,
  > {
    self.query_inner(database, query, _type, env).await
  }

  #[cfg(feature = "native")]
  pub async fn query(
    &mut self,
    database: String,
    query: String,
    _type: Option<QueryType>,
    env: &Env,
  ) -> napi::Result<
    Either3<
      ReadableStream<'_, serde_json::Value>,
      ReadableStream<'_, HashMap<String, Option<Value>>>,
      ReadableStream<'_, Vec<u8>>,
    >,
  > {
    self.query_inner(database, query, _type, env).await
  }

  async fn write_inner(
    &mut self,
    lines: Vec<String>,
    database: String,
    write_options: Option<WriteOptions>,
    org: Option<String>,
  ) -> napi::Result<()> {
    let (url, write_options) = get_write_path(&self.addr, database, org, write_options)?;

    let headers = to_header_map(&write_options.headers.unwrap_or_default()).unwrap();
    let response = &self
      .http_client
      .post(url)
      .body(lines.join("\n"))
      .headers(headers)
      .send()
      .await;

    match response {
      Ok(response) => {
        println!("{response:?}");
        Ok(())
      }
      Err(error) => {
        println!("Error occurred: {error:?}");
        Err(napi::Error::from_status(Status::Cancelled))
      }
    }
  }

  #[cfg(feature = "native")]
  pub async fn write(
    &mut self,
    lines: Vec<String>,
    database: String,
    write_options: Option<WriteOptions>,
    org: Option<String>,
  ) -> napi::Result<()> {
    self.write_inner(lines, database, write_options, org).await
  }

  #[cfg(not(feature = "native"))]
  #[napi_derive::napi]
  /// # Safety
  ///
  /// This function should not be called before the horsemen are ready.
  pub async unsafe fn write(
    &mut self,
    lines: Vec<String>,
    database: String,
    write_options: Option<WriteOptions>,
    org: Option<String>,
  ) -> napi::Result<()> {
    self.write_inner(lines, database, write_options, org).await
  }
}

pub fn get_http_client(token: String) -> Client {
  let mut headers = HeaderMap::with_capacity(2);
  headers.insert(
    reqwest::header::AUTHORIZATION,
    reqwest::header::HeaderValue::from_str(format!("Bearer {token}").as_str()).expect("REASON"),
  );
  headers.insert(
    reqwest::header::CONTENT_TYPE,
    reqwest::header::HeaderValue::from_static("text/plain; charset=utf-8"),
  );
  Client::builder()
    .default_headers(headers)
    // .min_tls_version(Version::TLS_1_3)
    // .use_rustls_tls()
    .build()
    .unwrap()
}

fn get_channel(addr: String) -> Endpoint {
  Channel::from_shared(addr)
    .unwrap()
    .keep_alive_while_idle(true)
    .http2_keep_alive_interval(Duration::from_secs(5))
    .keep_alive_timeout(Duration::from_secs(10))
    .tls_config(tonic::transport::ClientTlsConfig::new().with_webpki_roots())
    .unwrap()
}

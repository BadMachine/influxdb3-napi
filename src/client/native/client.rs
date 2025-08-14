use crate::client::channel::get_channel;
use crate::client::http_client::get_http_client;
pub use crate::client::options::{to_header_map, FlightOptions, QueryPayload, WriteOptions};
use arrow_flight::{FlightClient, Ticket};
use napi::bindgen_prelude::*;
use napi::Status;
use reqwest::Client;
use tonic::codegen::Bytes;

use crate::query::common::query_processor::into_stream;
use crate::serializer::common::library_serializer::{LibraryReturnType, LibrarySerializer};
use crate::serializer::common::raw_serializer::RawSerializer;
use crate::serializer::common::unsafe_serializer::UnsafeSerializer;
use crate::serializer::common::{Serializer, SerializerTrait};
use crate::write::get_write_path;

pub struct InfluxDBClient {
  addr: String,
  flight_client: FlightClient,
  serializer: Serializer,
  http_client: Client,
}

impl InfluxDBClient {
  pub fn new(
    addr: String,
    token: Option<String>,
    serializer: Option<Serializer>,
    options: Option<FlightOptions>,
  ) -> Self {
    let channel = get_channel(addr.clone(), options).connect_lazy();

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
      http_client,
      serializer: serializer.unwrap_or(Serializer::Unsafe),
    }
  }

  pub async fn query(
    &mut self,
    query_payload: QueryPayload,
  ) -> Result<
    Either3<
      napi::tokio_stream::wrappers::ReceiverStream<Result<LibraryReturnType>>,
      napi::tokio_stream::wrappers::ReceiverStream<
        Result<serde_json::Map<String, serde_json::Value>>,
      >,
      napi::tokio_stream::wrappers::ReceiverStream<Result<Buffer>>,
    >,
  > {
    match self.serializer {
      Serializer::Library => {
        let stream = self.query_inner::<LibrarySerializer>(query_payload).await;
        Ok(Either3::A(stream))
      }
      Serializer::Unsafe => {
        let stream = self.query_inner::<UnsafeSerializer>(query_payload).await;
        Ok(Either3::B(stream))
      }
      Serializer::Raw => {
        let stream = self.query_inner::<RawSerializer>(query_payload).await;
        Ok(Either3::C(stream))
      }
    }
  }

  pub async fn query_inner<S: SerializerTrait>(
    &mut self,
    query_payload: QueryPayload,
  ) -> napi::tokio_stream::wrappers::ReceiverStream<napi::Result<<S as SerializerTrait>::Output>>
  {
    let payload: String = query_payload.into();

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    let response = self.flight_client.do_get(ticket).await;
    into_stream::<S>(response.unwrap())
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

    if let Err(e) = response {
      match e.status() {
        Some(reqwest::StatusCode::UNAUTHORIZED) => Err(napi::Error::from_reason("Unauthorized")),
        _ => Err(napi::Error::from_status(Status::Cancelled)),
      }
    } else {
      match response {
        Ok(response) => match response.status() {
          reqwest::StatusCode::OK => Ok(()),
          reqwest::StatusCode::NO_CONTENT => Ok(()),
          reqwest::StatusCode::UNAUTHORIZED => Err(napi::Error::from_reason("Unauthorized")),
          _ => Err(napi::Error::from_reason("Unknown")),
        },
        Err(_error) => Err(napi::Error::from_status(Status::Cancelled)),
      }
    }
  }

  pub async fn write(
    &mut self,
    lines: Vec<String>,
    database: String,
    write_options: Option<WriteOptions>,
    org: Option<String>,
  ) -> napi::Result<()> {
    self.write_inner(lines, database, write_options, org).await
  }
}

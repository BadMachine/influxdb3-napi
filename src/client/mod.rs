mod channel;
mod http_client;
pub mod options;

use crate::client::channel::get_channel;
use crate::client::http_client::get_http_client;
use crate::client::options::QueryType;
pub use crate::client::options::{to_header_map, WriteOptions};
use crate::client::options::{FlightOptions, QueryPayload};
use crate::query::query_processor::into_stream;
use crate::serializer::library_serializer::{LibraryReturnType, LibrarySerializer};
use crate::serializer::Serializer;
use crate::serializer::SerializerTrait;
use crate::write::get_write_path;
use crate::{Status, Value};
use arrow_flight::{FlightClient, Ticket};
use napi::bindgen_prelude::{block_on, Either3, ReadableStream};

use crate::serializer::raw_serializer::RawSerializer;
use crate::serializer::unsafe_serializer::UnsafeSerializer;

use napi::Env;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tonic::codegen::Bytes;
use tonic::transport::{Channel, Endpoint};

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
pub struct InfluxDBClient {
  addr: String,
  flight_client: FlightClient,
  serializer: Serializer,
  http_client: Client,
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl InfluxDBClient {
  #[cfg_attr(not(feature = "native"), napi_derive::napi(constructor))]
  pub fn new(
    addr: String,
    token: Option<String>,
    serializer: Option<Serializer>,
    options: Option<FlightOptions>,
  ) -> Self {
    #[cfg(not(feature = "native"))]
    use napi::bindgen_prelude::block_on;
    #[cfg(not(feature = "native"))]
    let channel = block_on(async {
      get_channel(addr.clone(), options)
        .connect()
        .await
        .expect("error connecting")
    });

    #[cfg(feature = "native")]
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

  #[cfg(feature = "native")]
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

  #[cfg(not(feature = "native"))]
  pub fn query_inner<S: SerializerTrait>(
    &mut self,
    query_payload: QueryPayload,
    env: &Env,
  ) -> napi::Result<ReadableStream<'_, S::Output>> {
    let payload: String = query_payload.into();

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    use napi::bindgen_prelude::block_on;

    let stream = block_on(async {
      let response = self.flight_client.do_get(ticket).await;
      into_stream::<S>(response.unwrap())
    });

    ReadableStream::new(env, stream)
  }

  #[cfg(not(feature = "native"))]
  #[napi_derive::napi]
  pub fn query(
    &mut self,
    query_payload: QueryPayload,
    env: &Env,
  ) -> napi::Result<
    Either3<
      ReadableStream<'_, LibraryReturnType>, // Library
      ReadableStream<'_, serde_json::Map<String, serde_json::Value>>, // Unsafe
      ReadableStream<'_, napi::bindgen_prelude::Buffer>, // Raw
    >,
  > {
    match self.serializer {
      Serializer::Library => {
        let stream = self.query_inner::<LibrarySerializer>(query_payload, env)?;
        Ok(Either3::A(stream))
      }
      Serializer::Unsafe => {
        let stream = self.query_inner::<UnsafeSerializer>(query_payload, env)?;
        Ok(Either3::B(stream))
      }
      Serializer::Raw => {
        let stream = self.query_inner::<RawSerializer>(query_payload, env)?;
        Ok(Either3::C(stream))
      }
    }
  }

  #[cfg(feature = "native")]
  pub async fn query(
    &mut self,
    query_payload: QueryPayload,
  ) -> napi::Result<
    Either3<
      napi::tokio_stream::wrappers::ReceiverStream<napi::Result<LibraryReturnType>>, // Library
      napi::tokio_stream::wrappers::ReceiverStream<
        napi::Result<serde_json::Map<String, serde_json::Value>>,
      >, // Unsafe
      napi::tokio_stream::wrappers::ReceiverStream<napi::Result<napi::bindgen_prelude::Buffer>>, // Raw
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
        Some(reqwest::StatusCode::UNAUTHORIZED) => {
          Err(napi::Error::from_reason("Unauthorized").into())
        }
        _ => Err(napi::Error::from_status(Status::Cancelled)),
      }
    } else {
      match response {
        Ok(response) => match response.status() {
          reqwest::StatusCode::OK => Ok(()),
          reqwest::StatusCode::NO_CONTENT => Ok(()),
          reqwest::StatusCode::UNAUTHORIZED => Err(napi::Error::from_reason("Unauthorized").into()),
          _ => Err(napi::Error::from_reason("Unknown").into()),
        },
        Err(error) => Err(napi::Error::from_status(Status::Cancelled)),
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

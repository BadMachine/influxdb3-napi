use crate::client::channel::get_channel;
use crate::client::http_client::get_http_client;
pub use crate::client::options::{to_header_map, WriteOptions};
use crate::client::options::{FlightOptions, QueryPayload};
use crate::query::common::query_processor::into_stream;
use crate::serializer::common::library_serializer::{LibraryReturnType, LibrarySerializer};
use crate::serializer::common::raw_serializer::RawSerializer;
use crate::serializer::common::unsafe_serializer::UnsafeSerializer;
use crate::serializer::common::Serializer;
use crate::serializer::common::SerializerTrait;
use crate::write::get_write_path;
use crate::Status;
use arrow_flight::{FlightClient, Ticket};
use napi::bindgen_prelude::*;
use napi::tokio_stream::wrappers::ReceiverStream;
use napi::Env;
use reqwest::Client;
use tonic::codegen::Bytes;

#[napi_derive::napi]
pub struct InfluxDBClient {
  addr: String,
  flight_client: FlightClient,
  serializer: Serializer,
  http_client: Client,
}

#[napi_derive::napi]
impl InfluxDBClient {
  #[napi(constructor)]
  pub fn new(
    addr: String,
    token: Option<String>,
    serializer: Option<Serializer>,
    options: Option<FlightOptions>,
  ) -> Self {
    use napi::bindgen_prelude::block_on;
    let channel = block_on(async {
      get_channel(addr.clone(), options)
        .connect()
        .await
        .expect("error connecting")
    });

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

  #[allow(clippy::type_complexity)]
  #[napi_derive::napi]
  pub fn query(
    &mut self,
    query_payload: QueryPayload,
    env: &Env,
  ) -> napi::Result<
    Either3<
      ReadableStream<'_, LibraryReturnType>,
      ReadableStream<'_, serde_json::Map<String, serde_json::Value>>,
      ReadableStream<'_, Buffer>,
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

    let stream: ReceiverStream<Result<<S as SerializerTrait>::Output>> = block_on(async {
      let response = self.flight_client.do_get(ticket).await;
      into_stream::<S>(response.unwrap())
    });

    ReadableStream::new(env, stream)
  }

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
  ) -> Result<()> {
    self.write_inner(lines, database, write_options, org).await
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
}

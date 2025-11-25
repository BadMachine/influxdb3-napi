use arrow::array::RecordBatch;
use arrow_flight::error::Result as FlightResult;
use napi::bindgen_prelude::ToNapiValue;
use napi_derive::napi;
use std::future::Future;

pub mod library_serializer;
pub mod raw_serializer;
pub mod unsafe_serializer;

#[napi(string_enum)]
#[derive(Debug, Clone)]
pub enum Serializer {
  #[napi(value = "unsafe")]
  Unsafe,

  #[napi(value = "library")]
  Library,

  #[napi(value = "raw")]
  Raw,
}

impl Default for Serializer {
  fn default() -> Self {
    Self::Library
  }
}

pub trait SerializerTrait {
  type Output: ToNapiValue + Send + 'static;

  fn serialize(
    batch: FlightResult<RecordBatch>,
  ) -> impl Future<Output = Option<Vec<Self::Output>>> + Send;
}

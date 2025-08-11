use arrow::array::RecordBatch;
use std::future::Future;
// NAPI DOES NOT SUPPORT GENERIC STRUCTURE DEPS
use napi_derive::napi;
pub mod library_serializer;
pub mod raw_serializer;
pub mod unsafe_serializer;

use arrow_flight::error::Result as FlightResult;
use napi::bindgen_prelude::ToNapiValue;

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

pub trait SerializerTrait {
  type Output: ToNapiValue + Send + 'static;

  fn serialize(
    batch: FlightResult<RecordBatch>,
  ) -> impl Future<Output = Option<Vec<Self::Output>>> + Send;
}

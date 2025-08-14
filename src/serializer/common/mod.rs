use arrow_flight::error::Result as FlightResult;
use std::future::Future;
use arrow::array::RecordBatch;
use napi::bindgen_prelude::ToNapiValue;
use napi_derive::napi;

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

#[cfg(not(target_arch = "wasm32"))]
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
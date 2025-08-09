use std::future::Future;
use arrow::array::RecordBatch;
// NAPI DOES NOT SUPPORT GENERIC STRUCTURE DEPS
use napi_derive::napi;
pub mod unsafe_serializer;
pub mod library_serializer;

use arrow_flight::error::Result as FlightResult;
use napi::bindgen_prelude::ToNapiValue;
use crate::serializer::unsafe_serializer::UnsafeSerializer;

#[napi(string_enum)]
#[derive(Debug, Clone)]
pub enum Serializer {
    #[napi(value = "unsafe")]
    Unsafe,

    #[napi(value = "library")]
    Library,

    #[napi(value = "raw")]
    Raw
}



impl TryInto<UnsafeSerializer> for Serializer {
    fn try_into(self) -> Result<UnsafeSerializer, Self::Error> {
        if let Serializer::Unsafe = self {
            UnsafeSerializer {}
        }
    }
}

pub trait SerializerType {
    type Output: ToNapiValue + Send + 'static;

    fn serialize(batch: FlightResult<RecordBatch>) -> impl Future<Output = Option<Vec<Self::Output>>> + Send;
}
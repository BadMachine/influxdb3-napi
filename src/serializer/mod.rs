// NAPI DOES NOT SUPPORT GENERIC STUCTURE DEPS
use napi_derive::napi;
pub mod not_safe;
//
// use arrow::record_batch::RecordBatch;
// use napi::bindgen_prelude::ToNapiValue;
//
// pub(crate) trait Serializer {
//     async fn compute(batch: Option<arrow_flight::error::Result<RecordBatch>>) -> napi::Result<Option<impl ToNapiValue>>;
// }

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
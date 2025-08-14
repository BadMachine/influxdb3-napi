// CURRENTLY NAPI-RS DOES NOT SUPPORT TRAIT IMPLEMENTATIONS

// use napi::bindgen_prelude::{Either3, ReadableStream};
// use napi::Env;
// use crate::client::options::{FlightOptions, QueryPayload};
// use crate::client::WriteOptions;
// use crate::serializer::library_serializer::LibraryReturnType;
// use crate::serializer::Serializer;
// 
// pub trait InfluxClientTrait {
//     fn new(
//         addr: String,
//         token: Option<String>,
//         serializer: Option<Serializer>,
//         options: Option<FlightOptions>,
//     ) -> Self;
// 
//     fn query(
//         &mut self,
//         query_payload: QueryPayload,
//         env: &Env,
//     ) -> napi::Result<
//         Either3<
//             ReadableStream<'_, LibraryReturnType>,
//             ReadableStream<'_, serde_json::Map<String, serde_json::Value>>,
//             ReadableStream<'_, napi::bindgen_prelude::Buffer>,
//         >,
//     >;
// 
//     fn write(
//         &mut self,
//         lines: Vec<String>,
//         database: String,
//         write_options: Option<WriteOptions>,
//         org: Option<String>,
//     ) -> napi::Result<()>;
// }

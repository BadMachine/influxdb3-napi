use influxdb3_napi::client::options::{QueryPayload, QueryType};
use napi::bindgen_prelude::Either3;
use napi::tokio;
use napi::tokio_stream::StreamExt;

#[tokio::main]
#[cfg(feature = "native")]
async fn main() {
}

#[tokio::main]
#[cfg(not(feature = "native"))]
async fn main() {
  println!("Not implemented yet");
}

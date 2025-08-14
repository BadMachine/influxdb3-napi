use crate::client::browser::{HttpQueryResponseV1, Series};
use crate::serializer::browser::unsafe_serializer::UnsafeSerializer;
use napi::bindgen_prelude::*;
use napi::tokio_stream::wrappers::ReceiverStream;

pub(crate) fn into_stream(
  mut response: HttpQueryResponseV1,
) -> ReceiverStream<Result<serde_json::Map<String, serde_json::Value>>> {
  let (tx, rx) =
    tokio::sync::mpsc::channel::<Result<serde_json::Map<String, serde_json::Value>>>(100);

  tokio::spawn(async move {
    let HttpQueryResponseV1 { results } = response;

    if let Some(data) = results.first() {
      if let Some(series) = data.series.first() {
        let Series {
          columns, values, ..
        } = series;

        for item in values {
          if let Some(result) = UnsafeSerializer::serialize(columns, item).await {
            if tx.send(Ok(result)).await.is_err() {
              break;
            }
          }
        }
      }
    }
  });

  ReceiverStream::new(rx)
}

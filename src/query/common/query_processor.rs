use crate::serializer::common::SerializerTrait;

use arrow_flight::decode::FlightRecordBatchStream;
use napi::bindgen_prelude::*;
use napi::tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::tokio_stream::StreamExt;

pub(crate) fn into_stream<S: SerializerTrait>(
    mut response: FlightRecordBatchStream,
) -> ReceiverStream<Result<<S as SerializerTrait>::Output>> {
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<S::Output>>(100);

    tokio::spawn(async move {
        while let Some(batch) = response.next().await {
            let serialized_result = S::serialize(batch).await;

            if let Some(data) = serialized_result {
                for item in data {
                    if tx.send(Ok(item)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    ReceiverStream::new(rx)
}
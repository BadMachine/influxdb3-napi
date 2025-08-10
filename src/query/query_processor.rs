use crate::serializer::SerializerTrait;

use arrow_flight::decode::FlightRecordBatchStream;
use napi::bindgen_prelude::*;
use napi::tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::tokio_stream::StreamExt;

pub(crate) fn into_stream<S: SerializerTrait>(
  mut response: FlightRecordBatchStream,
) -> ReceiverStream<Result<<S as SerializerTrait>::Output>> {
  let (tx, rx) = tokio::sync::mpsc::channel::<Result<S::Output>>(100);

  napi::tokio::spawn(async move {
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
//
// #[derive(Debug)]
// pub struct QueryProcessor<S: SerializerTrait> {
//   pub(crate) response: FlightRecordBatchStream,
//   serializer: Serializer,
//   _phantom: std::marker::PhantomData<S>,
// }
//
// impl<S: SerializerTrait> QueryProcessor<S> {
//   pub fn new(response: FlightRecordBatchStream, serializer: Serializer) -> Self {
//     Self {
//       response,
//       serializer,
//       _phantom: std::marker::PhantomData,
//     }
//   }
//
//   // Синхронный метод, который создает ReadableStream
//   pub(crate) fn into_stream(mut self, env: &Env) -> Result<ReadableStream<'_, S::Output>> {
//     let (tx, rx) = tokio::sync::mpsc::channel::<Result<S::Output>>(100);
//
//     // Запускаем асинхронную обработку в отдельной задаче
//     tokio::spawn(async move {
//       while let Some(batch) = self.response.next().await {
//         let serialized_result = S::serialize(batch).await;
//
//         if let Some(data) = serialized_result {
//           for item in data {
//             if tx.send(Ok(item)).await.is_err() {
//               break;
//             }
//           }
//         }
//       }
//     });
//
//     ReadableStream::new(env, ReceiverStream::new(rx))
//   }
// pub(crate) async fn process(mut self, env: &Env) -> Result<ReadableStream<'static, S::Output>> {
//   let (tx, rx) = tokio::sync::mpsc::channel::<Result<S::Output>>(100);
//
//   while let Some(batch) = self.response.next().await {
//     let tx = tx.clone();
//
//     tokio::spawn(async move {
//       let serialized_result = S::serialize(batch).await;
//
//       if let Some(data) = serialized_result {
//         for item in data {
//
//           match tx.try_send(Ok(item)) {
//             Err(TrySendError::Closed(_)) => {
//               panic!("closed");
//             }
//             Err(TrySendError::Full(_)) => {
//               panic!("queue is full");
//             }
//             Ok(_) => {}
//           }
//         }
//       }
//     });
//   }
//
//   ReadableStream::new(env, ReceiverStream::new(rx))
// }
// }

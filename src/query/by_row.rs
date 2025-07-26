use arrow_flight::decode::FlightRecordBatchStream;
use std::collections::HashMap;
// use napi::bindgen_prelude::async_iterator::AsyncGenerator;
// use napi::bindgen_prelude::Generator;
use tokio_stream::StreamExt;

pub struct QueryResultByRow {
  pub(crate) response: FlightRecordBatchStream,
}

impl QueryResultByRow {
  // async fn get_by_index(&mut self, index: i32) -> napi::Result<Option<crate::ReturnDataType>> {
  //   let batch = self.response.next().await;
  //
  //   if let Some(Ok(batch)) = batch {
  //     let row_count = batch.num_rows();
  //
  //     if index >= row_count as i32 {
  //       Ok(None)
  //     } else {
  //       Ok(Some(HashMap::new()))
  //     }
  //   } else {
  //     Ok(None)
  //   }
  // }
}
//
// #[napi(async_iterator)]
// // #[napi]
// pub struct Fib {
//     current: u32,
//     next: u32,
// }
//
// #[napi]
// impl AsyncGenerator for Fib {
//     type Yield = u32;
//     type Next = i32;
//     type Return = ();
//
//     fn next(
//         &mut self,
//         value: Option<Self::Next>,
//     ) -> impl std::future::Future<Output = napi::Result<Option<Self::Yield>>> + Send + 'static {
//         if let Some(n) = value {
//             self.current = n as u32;
//             self.next = self.current + 1;
//         } else {
//             let next = self.next;
//             let current = self.current;
//             self.current = next;
//             self.next = current + next;
//         }
//
//         let result = Some(self.current);
//
//         async move {
//             Ok(result)
//         }
//     }
// }

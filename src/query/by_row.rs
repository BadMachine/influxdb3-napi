#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(clippy::disallowed_names)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::new_without_default)]
#![allow(deprecated)]

use arrow_flight::decode::FlightRecordBatchStream;

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


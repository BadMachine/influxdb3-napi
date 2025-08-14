use crate::serializer::common::SerializerTrait;
use arrow::array::RecordBatch;
use arrow_flight::error::Result as FlightResult;

#[cfg(not(target_arch = "wasm32"))]
pub struct UnsafeSerializer;
impl SerializerTrait for UnsafeSerializer {
  type Output = serde_json::Map<String, serde_json::Value>;

  async fn serialize(batch: FlightResult<RecordBatch>) -> Option<Vec<Self::Output>> {
    if let Ok(batch) = batch {
      serde_arrow::from_record_batch::<Vec<serde_json::Map<String, serde_json::Value>>>(&batch).ok()
    } else {
      None
    }
  }
}

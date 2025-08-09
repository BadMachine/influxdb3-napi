use arrow::array::RecordBatch;
use crate::serializer::SerializerType;
use arrow_flight::error::Result as FlightResult;

pub struct UnsafeSerializer;
impl SerializerType for UnsafeSerializer {
    type Output = serde_json::Value;

    async fn serialize(batch: FlightResult<RecordBatch>) -> Option<Vec<Self::Output>> {
        if let Ok(batch) = batch {
            serde_arrow::from_record_batch::<Vec<serde_json::Value>>(&batch).ok()
        } else {
            None
        }
    }
}


pub struct RawSerializer;
impl SerializerType for RawSerializer {
    type Output = String; // Или любой другой тип для Raw

    async fn serialize(batch: FlightResult<RecordBatch>) -> Option<Vec<Self::Output>> {
        if let Ok(batch) = batch {
            todo!("Implement raw serialization")
        } else {
            None
        }
    }
}
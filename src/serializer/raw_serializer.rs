use crate::serializer::FlightResult;
use arrow::array::RecordBatch;
use arrow::ipc::writer::StreamWriter;
use crate::serializer::SerializerTrait;

pub struct RawSerializer;

impl SerializerTrait for RawSerializer {
    type Output = Vec<u8>;

    async fn serialize(batch: FlightResult<RecordBatch>) -> Option<Vec<Self::Output>> {
        if let Ok(batch) = batch {
            match serialize_record_batch_to_bytes(&batch) {
                Ok(bytes) => Some(vec![bytes]),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

fn serialize_record_batch_to_bytes(batch: &RecordBatch) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    {
        let mut stream_writer = StreamWriter::try_new(&mut buffer, &batch.schema())?;
        stream_writer.write(batch)?;
        stream_writer.finish()?;
    }
    Ok(buffer)
}
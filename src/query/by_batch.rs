use arrow::array::{
  Array, BooleanArray, Date32Array, Date64Array,
  DurationMicrosecondArray, DurationMillisecondArray, DurationNanosecondArray, DurationSecondArray,
  Float16Array, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array,
  RecordBatch, StringArray, StringViewArray, Time32MillisecondArray, Time32SecondArray,
  Time64MicrosecondArray, Time64NanosecondArray, TimestampMicrosecondArray,
  TimestampMillisecondArray, TimestampNanosecondArray, TimestampSecondArray, UInt16Array,
  UInt32Array, UInt64Array, UInt8Array,
};
use arrow::datatypes::{DataType, TimeUnit};
use arrow_flight::decode::FlightRecordBatchStream;
use napi::tokio::spawn;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_stream::StreamExt;

use crate::Value;

#[derive(Debug)]
#[napi]
pub struct QueryResultByBatch {
  pub(crate) response: FlightRecordBatchStream,
}

#[napi]
impl QueryResultByBatch {
  pub fn new(response: FlightRecordBatchStream) -> Self {
    Self { response }
  }

  #[napi(js_name = "next")]
  /// # Safety
  ///
  /// This function should not be called before the horsemen are ready.
  pub async unsafe fn next(&mut self) -> napi::Result<Option<Vec<crate::ReturnDataType>>> {
    let batch = self.response.next().await;

    if let Some(Ok(batch)) = batch {
      let row_count = batch.num_rows();

      let arc_batch = Arc::new(batch);

      if row_count < 100 {
        let output =
          QueryResultByBatch::serialize_batch((0..row_count).collect(), Arc::clone(&arc_batch));
        return Ok(Some(output));
      }

      let threads_count = std::thread::available_parallelism()
        .map(|n| n.get().min(8))
        .unwrap_or(4);

      let chunk_size = row_count.div_ceil(threads_count);
      let mut handles = Vec::with_capacity(threads_count);

      for chunk_start in (0..row_count).step_by(chunk_size) {
        let chunk_end = (chunk_start + chunk_size).min(row_count);
        let indices: Vec<usize> = (chunk_start..chunk_end).collect();
        let batch_clone = Arc::clone(&arc_batch);

        handles.push(spawn(
          async { QueryResultByBatch::serialize_batch(indices, batch_clone) }, // async { QueryResult::serialize_batch_columnar( batch_clone) }
        ));
      }

      let mut output = Vec::with_capacity(row_count);
      for handle in handles {
        output.extend(handle.await.unwrap());
      }
      //
      // println!(
      //   "Serialization time threaded: {}ms",
      //   Instant::now().duration_since(start).as_millis()
      // );
      Ok(Some(output))
    } else {
      Ok(None)
    }
  }

  fn serialize_batch(
    indices: Vec<usize>,
    record_batch: Arc<RecordBatch>,
  ) -> Vec<crate::ReturnDataType> {
    let mut results = Vec::with_capacity(indices.len());

    for index in indices {
      let row = QueryResultByBatch::serialize(index, record_batch.clone());
      results.push(row);
    }

    results
  }

  fn serialize(index: usize, record_batch: Arc<RecordBatch>) -> crate::ReturnDataType {
    let mut return_map: crate::ReturnDataType = HashMap::new();

    let schema = record_batch.schema();

    for (f_index, field) in schema.fields().iter().enumerate() {
      let val_set = record_batch.column(f_index);
      let name = field.name().clone();

      let a = val_set.into_data();
      let value = match val_set.data_type() {
        DataType::Null => Some(Value::Null),

        DataType::Boolean => {
          let arr = BooleanArray::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Bool(arr.value(index)))
          }
        }
        DataType::Utf8View => {
          let arr = StringViewArray::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Text(arr.value(index).to_string()))
          }
        }
        DataType::Int8 => {
          let arr = Int8Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Int8(arr.value(index)))
          }
        }
        DataType::Int16 => {
          let arr = Int16Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Int16(arr.value(index)))
          }
        }
        DataType::Int32 => {
          let arr = Int32Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Int32(arr.value(index)))
          }
        }
        DataType::Int64 => {
          let arr = Int64Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Int64(arr.value(index)))
          }
        }
        DataType::UInt8 => {
          let arr = UInt8Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::U8(arr.value(index)))
          }
        }
        DataType::UInt16 => {
          let arr = UInt16Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::U16(arr.value(index)))
          }
        }
        DataType::UInt32 => {
          let arr = UInt32Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::U32(arr.value(index)))
          }
        }
        DataType::UInt64 => {
          let arr = UInt64Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::U64(arr.value(index)))
          }
        }
        DataType::Float16 => {
          let arr = Float16Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::F16(arr.value(index).to_f32()))
          }
        }
        DataType::Float32 => {
          let arr = Float32Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::F32(arr.value(index)))
          }
        }
        DataType::Float64 => {
          let arr = Float64Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::F64(arr.value(index)))
          }
        }
        DataType::Date32 => {
          let arr = Date32Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Date32(arr.value(index)))
          }
        }
        DataType::Date64 => {
          let arr = Date64Array::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::Date64(arr.value(index)))
          }
        }
        DataType::Utf8 | DataType::LargeUtf8 => {
          let arr = StringArray::from(a);
          if arr.is_null(index) {
            None
          } else {
            Some(Value::String(arr.value(index).to_string()))
          }
        }
        DataType::Duration(tu) => match tu {
          TimeUnit::Microsecond => {
            let arr = DurationMicrosecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("µs")))
            }
          }
          TimeUnit::Millisecond => {
            let arr = DurationMillisecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("ms")))
            }
          }
          TimeUnit::Nanosecond => {
            let arr = DurationNanosecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("ns")))
            }
          }
          TimeUnit::Second => {
            let arr = DurationSecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("s")))
            }
          }
        },
        DataType::Timestamp(tu, _) => match tu {
          TimeUnit::Microsecond => {
            let arr = TimestampMicrosecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("µs")))
            }
          }
          TimeUnit::Millisecond => {
            let arr = TimestampMillisecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("ms")))
            }
          }
          TimeUnit::Nanosecond => {
            let arr = TimestampNanosecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("ns")))
            }
          }
          TimeUnit::Second => {
            let arr = TimestampSecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("s")))
            }
          }
        },
        DataType::Time32(tu) => match tu {
          TimeUnit::Millisecond => {
            let arr = Time32MillisecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time32(arr.value(index), String::from("ms")))
            }
          }
          TimeUnit::Second => {
            let arr = Time32SecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time32(arr.value(index), String::from("s")))
            }
          }
          _ => Some(Value::String(String::from("<unsupported type>"))),
        },
        DataType::Time64(tu) => match tu {
          TimeUnit::Microsecond => {
            let arr = Time64MicrosecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("µs")))
            }
          }
          TimeUnit::Nanosecond => {
            let arr = Time64NanosecondArray::from(a);
            if arr.is_null(index) {
              None
            } else {
              Some(Value::Time64(arr.value(index), String::from("ns")))
            }
          }
          _ => Some(Value::String(String::from("<unsupported type>"))),
        },
        _ => Some(Value::Fallback),
      };

      return_map.insert(name, value);
    }

    return_map
  }
}

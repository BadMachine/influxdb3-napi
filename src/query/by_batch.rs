use crate::serializer::not_safe::unsafe_serialize;
use crate::serializer::Serializer;
use crate::Value;

use arrow::array::{
  Array, ArrayData, BooleanArray, Date32Array, Date64Array, DurationMicrosecondArray,
  DurationMillisecondArray, DurationNanosecondArray, DurationSecondArray, Float16Array,
  Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, RecordBatch,
  StringArray, StringViewArray, Time32MillisecondArray, Time32SecondArray, Time64MicrosecondArray,
  Time64NanosecondArray, TimestampMicrosecondArray, TimestampMillisecondArray,
  TimestampNanosecondArray, TimestampSecondArray, UInt16Array, UInt32Array, UInt64Array,
  UInt8Array,
};
use arrow::datatypes::{DataType, TimeUnit};
use arrow_flight::decode::FlightRecordBatchStream;
use napi::Either;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use napi::tokio::time::Instant;
use tokio_stream::StreamExt;

#[derive(Debug)]
#[cfg_attr(not(feature = "native"), napi)]
pub struct QueryResultByBatch {
  pub(crate) response: FlightRecordBatchStream,
  pub serializer: Serializer,
}

#[cfg_attr(not(feature = "native"), napi)]
impl QueryResultByBatch {
  pub fn new(response: FlightRecordBatchStream, serializer: Option<Serializer>) -> Self {
    Self {
      response,
      serializer: serializer.unwrap_or(Serializer::Unsafe),
    }
  }

  // /// # Safety
  // ///
  // /// This function should not be called before the horsemen are ready.
  // #[cfg_attr(not(feature = "native"), napi)]
  #[cfg(not(feature = "native"))]
  #[napi]
  pub async unsafe fn next(
    &mut self,
  ) -> napi::Result<Option<Either<Vec<crate::ReturnDataType>, Vec<serde_json::Value>>>> {
    let batch = self.response.next().await;

    match self.serializer {
      Serializer::Unsafe => {
        let serialized = unsafe_serialize(batch);

        match serialized {
          Some(s) => Ok(Some(Either::B(s))),
          None => Ok(None),
        }
      }
      Serializer::Library => {
        if let Some(Ok(batch)) = batch {
          let arc_batch = Arc::new(batch);

          let result = QueryResultByBatch::serialize_batch_columnwise(arc_batch).await;

          Ok(Some(Either::A(result)))
        } else {
          Ok(None)
        }
      }
      _ => Ok(None),
    }
  }

  // #[cfg(not(feature = "native"))]
  // #[napi]
  // /// # Safety
  // ///
  // /// This function should not be called before the horsemen are ready.
  // pub async unsafe fn next(
  //   &mut self,
  // ) -> napi::Result<Option<Either<Vec<crate::ReturnDataType>, Vec<serde_json::Value>>>> {
  //   let batch = self.response.next().await;
  //
  //   match self.serializer {
  //     Serializer::Unsafe => {
  //       let serialized = unsafe_serialize(batch);
  //
  //       match serialized {
  //         Some(s) => Ok(Some(Either::B(s))),
  //         None => Ok(None),
  //       }
  //     }
  //     Serializer::Library => {
  //       if let Some(Ok(batch)) = batch {
  //         let row_count = batch.num_rows();
  //
  //         let arc_batch = Arc::new(batch);
  //
  //         if row_count < 100 {
  //           let output =
  //             QueryResultByBatch::serialize_batch((0..row_count).collect(), Arc::clone(&arc_batch));
  //           return Ok(Some(Either::A(output)));
  //         }
  //
  //         let threads_count = std::thread::available_parallelism()
  //           .map(|n| n.get().min(8))
  //           .unwrap_or(4);
  //
  //         let chunk_size = row_count.div_ceil(threads_count);
  //         let mut handles = Vec::with_capacity(threads_count);
  //
  //         for chunk_start in (0..row_count).step_by(chunk_size) {
  //           let chunk_end = (chunk_start + chunk_size).min(row_count);
  //           let indices: Vec<usize> = (chunk_start..chunk_end).collect();
  //           let batch_clone = Arc::clone(&arc_batch);
  //
  //           handles.push(napi::tokio::spawn(async {
  //             QueryResultByBatch::serialize_batch(indices, batch_clone)
  //           }));
  //         }
  //
  //         let mut output = Vec::with_capacity(row_count);
  //         for handle in handles {
  //           output.extend(handle.await.unwrap());
  //         }
  //         //
  //         // println!(
  //         //   "Serialization time threaded: {}ms",
  //         //   Instant::now().duration_since(start).as_millis()
  //         // );
  //         Ok(Some(Either::A(output)))
  //       } else {
  //         Ok(None)
  //       }
  //     }
  //     _ => Ok(None),
  //   }
  // }

  #[cfg(feature = "native")]
  pub async fn next(
    &mut self,
  ) -> napi::Result<Option<Either<Vec<crate::ReturnDataType>, Vec<serde_json::Value>>>> {

    // let steam_yield_time = Instant::now();

    let batch = self.response.next().await;

    // println!("Yield batch time: {:?}", steam_yield_time.elapsed());

    match self.serializer {
      Serializer::Unsafe => {
        let serialized = unsafe_serialize(batch);

        match serialized {
          Some(s) => Ok(Some(Either::B(s))),
          None => Ok(None),
        }
      }
      Serializer::Library => {
        match batch {
          Some(batch) => {
            match batch {
              Ok(batch) => {
                let arc_batch = Arc::new(batch);

                let result = QueryResultByBatch::serialize_batch_columnwise(arc_batch).await;

                Ok(Some(Either::A(result)))
              },
              Err(e) => {
                println!("Error reading next batch: {:?}", e);
                Ok(None)
              }
            }
          },
          None => Ok(None)
        }
        // if let Some(Ok(batch)) = batch {
        //   let arc_batch = Arc::new(batch);
        //
        //   let result = QueryResultByBatch::serialize_batch_columnwise(arc_batch).await;
        //
        //   Ok(Some(Either::A(result)))
        // } else {
        //   Ok(None)
        // }
      }
      _ => Ok(None),
    }
  }

  pub async fn serialize_batch_columnwise(batch: Arc<RecordBatch>) -> Vec<crate::ReturnDataType> {
    let schema = batch.schema();
    let row_count = batch.num_rows();
    let field_count = schema.fields().len();

    let mut rows: Vec<crate::ReturnDataType> = (0..row_count)
      .map(|_| HashMap::with_capacity(field_count))
      .collect();

    let mut handles = Vec::with_capacity(field_count);

    // Запускаем параллельную обработку каждой колонки
    for (col_index, field) in schema.fields().iter().enumerate() {
      let column = batch.column(col_index).clone();
      let field_name = field.name().clone();

      let handle = napi::tokio::task::spawn_blocking(move || {
        Self::serialize_column(&column, field_name, row_count)
      });

      handles.push(handle);
    }

    for handle in handles {
      let (col_name, column_values) = handle.await.unwrap();
      for (row_idx, value) in column_values.into_iter().enumerate() {
        rows[row_idx].insert(col_name.clone(), value);
      }
    }

    rows
  }

  fn serialize_column(
    column: &Arc<dyn Array>,
    field_name: String,
    row_count: usize,
  ) -> (String, Vec<Option<Value>>) {
    let mut column_values = Vec::with_capacity(row_count);
    let array_data = column.into_data();

    match column.data_type() {
      DataType::Null => {
        column_values.resize(row_count, Some(Value::Null));
      }
      DataType::Boolean => {
        let arr = BooleanArray::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Bool(arr.value(i)))
          });
        }
      }
      DataType::Utf8View => {
        let arr = StringViewArray::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Text(arr.value(i).to_string()))
          });
        }
      }
      DataType::Int8 => {
        let arr = Int8Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Int8(arr.value(i)))
          });
        }
      }
      DataType::Int16 => {
        let arr = Int16Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Int16(arr.value(i)))
          });
        }
      }
      DataType::Int32 => {
        let arr = Int32Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Int32(arr.value(i)))
          });
        }
      }
      DataType::Int64 => {
        let arr = Int64Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Int64(arr.value(i)))
          });
        }
      }
      DataType::UInt8 => {
        let arr = UInt8Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::U8(arr.value(i)))
          });
        }
      }
      DataType::UInt16 => {
        let arr = UInt16Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::U16(arr.value(i)))
          });
        }
      }
      DataType::UInt32 => {
        let arr = UInt32Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::U32(arr.value(i)))
          });
        }
      }
      DataType::UInt64 => {
        let arr = UInt64Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::U64(arr.value(i)))
          });
        }
      }
      DataType::Float16 => {
        let arr = Float16Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::F16(arr.value(i).to_f32()))
          });
        }
      }
      DataType::Float32 => {
        let arr = Float32Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::F32(arr.value(i)))
          });
        }
      }
      DataType::Float64 => {
        let arr = Float64Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::F64(arr.value(i)))
          });
        }
      }
      DataType::Date32 => {
        let arr = Date32Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Date32(arr.value(i)))
          });
        }
      }
      DataType::Date64 => {
        let arr = Date64Array::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Date64(arr.value(i)))
          });
        }
      }
      DataType::Utf8 | DataType::LargeUtf8 => {
        let arr = StringArray::from(array_data);
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::String(arr.value(i).to_string()))
          });
        }
      }
      DataType::Duration(time_unit) => {
        Self::serialize_duration_column(&array_data, time_unit, row_count, &mut column_values);
      }
      DataType::Timestamp(time_unit, _) => {
        Self::serialize_timestamp_column(&array_data, time_unit, row_count, &mut column_values);
      }
      DataType::Time32(time_unit) => {
        Self::serialize_time32_column(&array_data, time_unit, row_count, &mut column_values);
      }
      DataType::Time64(time_unit) => {
        Self::serialize_time64_column(&array_data, time_unit, row_count, &mut column_values);
      }
      _ => {
        column_values.resize(row_count, Some(Value::Fallback));
      }
    }

    (field_name, column_values)
  }

  fn serialize_duration_column(
    array_data: &ArrayData,
    time_unit: &TimeUnit,
    row_count: usize,
    column_values: &mut Vec<Option<Value>>,
  ) {
    match time_unit {
      TimeUnit::Microsecond => {
        let arr = DurationMicrosecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "µs".to_string()))
          });
        }
      }
      TimeUnit::Millisecond => {
        let arr = DurationMillisecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "ms".to_string()))
          });
        }
      }
      TimeUnit::Nanosecond => {
        let arr = DurationNanosecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "ns".to_string()))
          });
        }
      }
      TimeUnit::Second => {
        let arr = DurationSecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "s".to_string()))
          });
        }
      }
    }
  }

  fn serialize_timestamp_column(
    array_data: &ArrayData,
    time_unit: &TimeUnit,
    row_count: usize,
    column_values: &mut Vec<Option<Value>>,
  ) {
    match time_unit {
      TimeUnit::Microsecond => {
        let arr = TimestampMicrosecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "µs".to_string()))
          });
        }
      }
      TimeUnit::Millisecond => {
        let arr = TimestampMillisecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "ms".to_string()))
          });
        }
      }
      TimeUnit::Nanosecond => {
        let arr = TimestampNanosecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "ns".to_string()))
          });
        }
      }
      TimeUnit::Second => {
        let arr = TimestampSecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "s".to_string()))
          });
        }
      }
    }
  }

  fn serialize_time32_column(
    array_data: &ArrayData,
    time_unit: &TimeUnit,
    row_count: usize,
    column_values: &mut Vec<Option<Value>>,
  ) {
    match time_unit {
      TimeUnit::Millisecond => {
        let arr = Time32MillisecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time32(arr.value(i), "ms".to_string()))
          });
        }
      }
      TimeUnit::Second => {
        let arr = Time32SecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time32(arr.value(i), "s".to_string()))
          });
        }
      }
      _ => {
        column_values.resize(
          row_count,
          Some(Value::String("<unsupported type>".to_string())),
        );
      }
    }
  }

  fn serialize_time64_column(
    array_data: &ArrayData,
    time_unit: &TimeUnit,
    row_count: usize,
    column_values: &mut Vec<Option<Value>>,
  ) {
    match time_unit {
      TimeUnit::Microsecond => {
        let arr = Time64MicrosecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "µs".to_string()))
          });
        }
      }
      TimeUnit::Nanosecond => {
        let arr = Time64NanosecondArray::from(array_data.clone());
        for i in 0..row_count {
          column_values.push(if arr.is_null(i) {
            None
          } else {
            Some(Value::Time64(arr.value(i), "ns".to_string()))
          });
        }
      }
      _ => {
        column_values.resize(
          row_count,
          Some(Value::String("<unsupported type>".to_string())),
        );
      }
    }

    // fn serialize(index: usize, record_batch: &Arc<RecordBatch>) -> crate::ReturnDataType {
    //   let mut return_map: crate::ReturnDataType = HashMap::new();
    //
    //   let schema = record_batch.schema();
    //   let columns = record_batch.columns();
    //
    //   for (f_index, field) in schema.fields().iter().enumerate() {
    //     let val_set = &columns[f_index];
    //     let name = field.name().clone();
    //
    //     let a = val_set.into_data();
    //     let value = match val_set.data_type() {
    //       DataType::Null => Some(Value::Null),
    //
    //       DataType::Boolean => {
    //         let arr = BooleanArray::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Bool(arr.value(index)))
    //         }
    //       }
    //       DataType::Utf8View => {
    //         let arr = StringViewArray::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Text(arr.value(index).to_string()))
    //         }
    //       }
    //       DataType::Int8 => {
    //         let arr = Int8Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Int8(arr.value(index)))
    //         }
    //       }
    //       DataType::Int16 => {
    //         let arr = Int16Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Int16(arr.value(index)))
    //         }
    //       }
    //       DataType::Int32 => {
    //         let arr = Int32Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Int32(arr.value(index)))
    //         }
    //       }
    //       DataType::Int64 => {
    //         let arr = Int64Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Int64(arr.value(index)))
    //         }
    //       }
    //       DataType::UInt8 => {
    //         let arr = UInt8Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::U8(arr.value(index)))
    //         }
    //       }
    //       DataType::UInt16 => {
    //         let arr = UInt16Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::U16(arr.value(index)))
    //         }
    //       }
    //       DataType::UInt32 => {
    //         let arr = UInt32Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::U32(arr.value(index)))
    //         }
    //       }
    //       DataType::UInt64 => {
    //         let arr = UInt64Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::U64(arr.value(index)))
    //         }
    //       }
    //       DataType::Float16 => {
    //         let arr = Float16Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::F16(arr.value(index).to_f32()))
    //         }
    //       }
    //       DataType::Float32 => {
    //         let arr = Float32Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::F32(arr.value(index)))
    //         }
    //       }
    //       DataType::Float64 => {
    //         let arr = Float64Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::F64(arr.value(index)))
    //         }
    //       }
    //       DataType::Date32 => {
    //         let arr = Date32Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Date32(arr.value(index)))
    //         }
    //       }
    //       DataType::Date64 => {
    //         let arr = Date64Array::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::Date64(arr.value(index)))
    //         }
    //       }
    //       DataType::Utf8 | DataType::LargeUtf8 => {
    //         let arr = StringArray::from(a);
    //         if arr.is_null(index) {
    //           None
    //         } else {
    //           Some(Value::String(arr.value(index).to_string()))
    //         }
    //       }
    //       DataType::Duration(tu) => match tu {
    //         TimeUnit::Microsecond => {
    //           let arr = DurationMicrosecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("µs")))
    //           }
    //         }
    //         TimeUnit::Millisecond => {
    //           let arr = DurationMillisecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("ms")))
    //           }
    //         }
    //         TimeUnit::Nanosecond => {
    //           let arr = DurationNanosecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("ns")))
    //           }
    //         }
    //         TimeUnit::Second => {
    //           let arr = DurationSecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("s")))
    //           }
    //         }
    //       },
    //       DataType::Timestamp(tu, _) => match tu {
    //         TimeUnit::Microsecond => {
    //           let arr = TimestampMicrosecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("µs")))
    //           }
    //         }
    //         TimeUnit::Millisecond => {
    //           let arr = TimestampMillisecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("ms")))
    //           }
    //         }
    //         TimeUnit::Nanosecond => {
    //           let arr = TimestampNanosecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("ns")))
    //           }
    //         }
    //         TimeUnit::Second => {
    //           let arr = TimestampSecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("s")))
    //           }
    //         }
    //       },
    //       DataType::Time32(tu) => match tu {
    //         TimeUnit::Millisecond => {
    //           let arr = Time32MillisecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time32(arr.value(index), String::from("ms")))
    //           }
    //         }
    //         TimeUnit::Second => {
    //           let arr = Time32SecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time32(arr.value(index), String::from("s")))
    //           }
    //         }
    //         _ => Some(Value::String(String::from("<unsupported type>"))),
    //       },
    //       DataType::Time64(tu) => match tu {
    //         TimeUnit::Microsecond => {
    //           let arr = Time64MicrosecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("µs")))
    //           }
    //         }
    //         TimeUnit::Nanosecond => {
    //           let arr = Time64NanosecondArray::from(a);
    //           if arr.is_null(index) {
    //             None
    //           } else {
    //             Some(Value::Time64(arr.value(index), String::from("ns")))
    //           }
    //         }
    //         _ => Some(Value::String(String::from("<unsupported type>"))),
    //       },
    //       _ => Some(Value::Fallback),
    //     };
    //
    //     return_map.insert(name, value);
    //   }
    //
    //   return_map
    // }
  }
}

use crate::serializer::{FlightResult, SerializerTrait};
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
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;

pub struct LibrarySerializer;

#[napi]
pub type LibraryReturnType = HashMap<String, Option<Value>>;

impl SerializerTrait for LibrarySerializer {
  type Output = LibraryReturnType;

  async fn serialize(batch: FlightResult<RecordBatch>) -> Option<Vec<Self::Output>> {
    if let Ok(batch) = batch {
      let schema = batch.schema();
      let row_count = batch.num_rows();
      let field_count = schema.fields().len();

      let mut rows: Vec<Self::Output> = (0..row_count)
        .map(|_| HashMap::with_capacity(field_count))
        .collect();

      let mut handles = Vec::with_capacity(field_count);

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

      Some(rows)
    } else {
      None
    }
  }
}

impl LibrarySerializer {
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
  }
}

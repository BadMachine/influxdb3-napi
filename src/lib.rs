#![deny(clippy::all)]

use std::collections::HashMap;
use napi::bindgen_prelude::{Null, ToNapiValue};
use napi::sys::{napi_env, napi_value};

pub mod client;
pub mod query;

#[cfg_attr(feature = "napi", napi_derive::napi)]
pub type ReturnDataType = HashMap<String, Option<Value>>;

// #[derive(Clone)]
#[derive(Debug)]
pub enum Value {
  Time32(i32, String),
  Time64(i64, String),
  Int8(i8),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  U128(u128),
  F16(f32),
  F32(f32),
  F64(f64),
  Text(String),
  String(String),
  Bool(bool),
  Date32(i32),
  Date64(i64),
  Null,
  Fallback,
}

static FALLBACK_STR: &str = "<unsupported type>";

#[cfg_attr(feature = "napi", napi_derive::napi)]
impl ToNapiValue for Value {
  unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
    match val {
      Value::Int8(i) => ToNapiValue::to_napi_value(env, Ok(i)),
      Value::Int16(i) => ToNapiValue::to_napi_value(env, Ok(i)),
      Value::Int32(i) | Value::Date32(i) => ToNapiValue::to_napi_value(env, Ok(i)),
      Value::Int64(i) | Value::Date64(i) => ToNapiValue::to_napi_value(env, Ok(i)),
      Value::U8(bi) => ToNapiValue::to_napi_value(env, Ok(bi)),
      Value::U16(bi) => ToNapiValue::to_napi_value(env, Ok(bi)),
      Value::U32(bi) => ToNapiValue::to_napi_value(env, Ok(bi)),
      Value::U64(bi) => ToNapiValue::to_napi_value(env, Ok(bi)),
      Value::U128(bi) => ToNapiValue::to_napi_value(env, Ok(bi)),
      Value::F16(f) => ToNapiValue::to_napi_value(env, Ok(f)),
      Value::F32(f) => ToNapiValue::to_napi_value(env, Ok(f)),
      Value::F64(f) => ToNapiValue::to_napi_value(env, Ok(f)),
      Value::Text(t) => ToNapiValue::to_napi_value(env, Ok(t)),
      Value::Null => ToNapiValue::to_napi_value(env, Ok(Null)),
      Value::Bool(bv) => ToNapiValue::to_napi_value(env, Ok(bv)),
      Value::Time32(value, unit) => {
        ToNapiValue::to_napi_value(env, Ok(format!("{}_{}", value, unit)))
      }
      Value::Time64(value, unit) => {
        ToNapiValue::to_napi_value(env, Ok(format!("{}_{}", value, unit)))
      }
      Value::String(s) => ToNapiValue::to_napi_value(env, s),
      Value::Fallback => ToNapiValue::to_napi_value(env, Ok(FALLBACK_STR)),
    }
  }
}

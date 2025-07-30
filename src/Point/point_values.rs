use napi::bindgen_prelude::Either5;

use napi_derive::napi;
use std::collections::{BTreeMap, HashMap};

#[napi(string_enum)]
#[derive(Debug, Clone, PartialEq)]
pub enum PointFieldType {
  #[napi(value = "float")]
  Float,
  #[napi(value = "integer")]
  Integer,
  #[napi(value = "uinteger")]
  UInteger,
  #[napi(value = "string")]
  String,
  #[napi(value = "boolean")]
  Boolean,
}

#[napi(discriminant = "type")]
pub enum FieldEntry {
  Float(PointFieldType, f64),
  Integer(PointFieldType, i64),
  UInteger(PointFieldType, u32),
  String(PointFieldType, String),
  Boolean(PointFieldType, bool),
}

impl FieldEntry {
  pub fn get_type(&self) -> &PointFieldType {
    match self {
      FieldEntry::Float(t, _) => t,
      FieldEntry::Integer(t, _) => t,
      FieldEntry::UInteger(t, _) => t,
      FieldEntry::String(t, _) => t,
      FieldEntry::Boolean(t, _) => t,
    }
  }
}

#[cfg_attr(not(feature = "native"), napi)]
pub struct PointValues {
  pub(crate) name: Option<String>,
  time: Option<u32>,
  tags: BTreeMap<String, String>,
  fields: BTreeMap<String, FieldEntry>, //BTreeMap
}

#[cfg_attr(not(feature = "native"), napi)]
impl PointValues {
  #[cfg_attr(not(feature = "native"), napi(getter))]
  pub fn measurement(&self) -> Option<String> {
    self.name.clone()
  }

  #[cfg_attr(not(feature = "native"), napi(getter))]
  pub fn timestamp(&self) -> Option<u32> {
    self.time
  }

  #[cfg_attr(not(feature = "native"), napi_derive::napi)]
  pub fn set_timestamp(&mut self, time: u32) {
    self.time = Some(time);
  }

  pub fn get_fields(&self) -> &BTreeMap<String, FieldEntry> {
    &self.fields
  }


  #[cfg_attr(not(feature = "native"), napi(constructor))]
  pub fn new(measurement: String) -> Self {
    Self {
      name: Some(measurement),
      time: None,
      tags: BTreeMap::new(),
      fields: BTreeMap::new(),
    }
  }

  #[cfg_attr(not(feature = "native"), napi(factory))]
  pub fn from_measurement(measurement: String) -> Self {
    Self {
      name: Some(measurement),
      time: None,
      tags: BTreeMap::new(),
      fields: BTreeMap::new(),
    }
  }

  pub fn get_timestamp(&self) -> Option<u32> {
    self.time
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_tag(&self, tag_name: String) -> Option<String> {
    self.tags.get(&tag_name).cloned()
  }

  #[cfg_attr(not(feature = "native"), napi_derive::napi)]
  pub fn set_tag(&mut self, tag_name: String, tag_value: String) {
    self.tags.insert(tag_name, tag_value);
  }

  #[cfg_attr(not(feature = "native"), napi_derive::napi)]
  pub fn remove_tag(&mut self, tag_name: String) {
    self.tags.remove(&tag_name);
  }

  #[cfg_attr(not(feature = "native"), napi_derive::napi(getter))]
  pub fn tag_names(&self) -> Vec<String> {
    self.tags.keys().cloned().collect()
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_float_field(&self, name: String) -> napi::Result<Option<f64>> {
    match self.get_field(name, Some(PointFieldType::Float))? {
      Some(Either5::B(value)) => Ok(Some(*value)),
      Some(_) => Err(napi::Error::from_reason("Field exists but is not a float")),
      None => Ok(None),
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_float_field(&mut self, name: String, value: f64) {
    self
      .fields
      .insert(name, FieldEntry::Float(PointFieldType::Float, value));
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_int_field(&self, name: String) -> napi::Result<Option<i64>> {
    match self.get_field(name, Some(PointFieldType::Integer))? {
      Some(Either5::D(value)) => Ok(Some(*value)),
      Some(_) => Err(napi::Error::from_reason(
        "Field exists but is not an integer",
      )),
      None => Ok(None),
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_int_field(&mut self, name: String, value: i64) {
    self
      .fields
      .insert(name, FieldEntry::Integer(PointFieldType::Integer, value));
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_uinteger_field(&self, name: String) -> napi::Result<Option<u32>> {
    match self.get_field(name, Some(PointFieldType::UInteger))? {
      Some(Either5::C(value)) => Ok(Some(*value)),
      Some(_) => Err(napi::Error::from_reason(
        "Field exists but is not an unsigned integer",
      )),
      None => Ok(None),
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_uinteger_field(&mut self, name: String, value: u32) {
    self
      .fields
      .insert(name, FieldEntry::UInteger(PointFieldType::UInteger, value));
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_string_field(&self, name: String) -> napi::Result<Option<String>> {
    match self.get_field(name, Some(PointFieldType::String))? {
      Some(Either5::E(value)) => Ok(Some(value.clone())),
      Some(_) => Err(napi::Error::from_reason("Field exists but is not a string")),
      None => Ok(None),
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_string_field(&mut self, name: String, value: String) {
    self
      .fields
      .insert(name, FieldEntry::String(PointFieldType::String, value));
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_boolean_field(&self, name: String) -> napi::Result<Option<bool>> {
    match self.get_field(name, Some(PointFieldType::Boolean))? {
      Some(Either5::A(value)) => Ok(Some(*value)),
      Some(_) => Err(napi::Error::from_reason(
        "Field exists but is not a boolean",
      )),
      None => Ok(None),
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_boolean_field(&mut self, name: String, value: bool) {
    self
      .fields
      .insert(name, FieldEntry::Boolean(PointFieldType::Boolean, value));
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_field_type(&self, name: String) -> Option<PointFieldType> {
    match self.fields.get(&name) {
      Some(entry) => Some(entry.get_type().clone()),
      None => None,
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_field(
    &self,
    name: String,
    expected_type: Option<PointFieldType>,
  ) -> napi::Result<Option<Either5<&bool, &f64, &u32, &i64, &String>>> {
    let field_entry = self.fields.get(&name);

    match field_entry {
      Some(field_entry) => {
        if let Some(expected) = expected_type {
          if *field_entry.get_type() != expected {
            return Err(napi::Error::from_reason(format!(
              "Field '{}' exists but has type {:?}, expected {:?}",
              name,
              field_entry.get_type(),
              expected
            )));
          }
        }

        // Возвращаем значение в соответствующем варианте Either5
        match field_entry {
          FieldEntry::Boolean(_, value) => Ok(Some(Either5::A(value))),
          FieldEntry::Float(_, value) => Ok(Some(Either5::B(value))),
          FieldEntry::UInteger(_, value) => Ok(Some(Either5::C(value))),
          FieldEntry::Integer(_, value) => Ok(Some(Either5::D(value))),
          FieldEntry::String(_, value) => Ok(Some(Either5::E(value))),
        }
      }
      None => Ok(None), // Поле не найдено
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_field(
    &mut self,
    name: String,
    value: Either5<bool, f64, u32, i64, String>,
    field_type: Option<PointFieldType>,
  ) -> napi::Result<()> {
    let field_entry = match value {
      Either5::A(bool_value) => {
        let field_type = field_type.unwrap_or(PointFieldType::Boolean);
        if field_type != PointFieldType::Boolean {
          return Err(napi::Error::from_reason(format!(
            "Type mismatch: provided boolean value but expected type {:?}",
            field_type
          )));
        }
        FieldEntry::Boolean(field_type, bool_value)
      }
      Either5::B(float_value) => {
        let field_type = field_type.unwrap_or(PointFieldType::Float);
        if field_type != PointFieldType::Float {
          return Err(napi::Error::from_reason(format!(
            "Type mismatch: provided float value but expected type {:?}",
            field_type
          )));
        }
        FieldEntry::Float(field_type, float_value)
      }
      Either5::C(uint_value) => {
        let field_type = field_type.unwrap_or(PointFieldType::UInteger);
        if field_type != PointFieldType::UInteger {
          return Err(napi::Error::from_reason(format!(
            "Type mismatch: provided unsigned integer value but expected type {:?}",
            field_type
          )));
        }
        FieldEntry::UInteger(field_type, uint_value)
      }
      Either5::D(int_value) => {
        let field_type = field_type.unwrap_or(PointFieldType::Integer);
        if field_type != PointFieldType::Integer {
          return Err(napi::Error::from_reason(format!(
            "Type mismatch: provided integer value but expected type {:?}",
            field_type
          )));
        }
        FieldEntry::Integer(field_type, int_value)
      }
      Either5::E(string_value) => {
        let field_type = field_type.unwrap_or(PointFieldType::String);
        if field_type != PointFieldType::String {
          return Err(napi::Error::from_reason(format!(
            "Type mismatch: provided string value but expected type {:?}",
            field_type
          )));
        }
        FieldEntry::String(field_type, string_value)
      }
    };

    self.fields.insert(name, field_entry);
    Ok(())
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_fields(
    &mut self,
    values: HashMap<String, Either5<bool, f64, u32, i64, String>>,
  ) -> napi::Result<()> {
    for (name, value) in values {
      self.set_field(name, value, None)?
    }
    Ok(())
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn remove_field(&mut self, name: String) -> napi::Result<()> {
    match self.fields.remove(&name) {
      Some(_) => Ok(()),
      None => Err(napi::Error::from_reason(format!(
        "Field '{}' not found",
        name
      ))),
    }
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_field_names(&self) -> Vec<String> {
    self.fields.keys().cloned().collect()
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn has_fields(&self) -> bool {
    !self.fields.is_empty()
  }
}

// impl Default for PointValues {
//     fn default() -> Self {
//         Self {
//             name: None,
//             time: None,
//             tags: HashMap::new(),
//             fields: HashMap::new(),
//         }
//     }
// }

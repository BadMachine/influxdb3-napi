mod escape;
pub mod point_values;

use crate::client::options::TimeUnitV2;
use crate::Point::escape::Escaper;
use crate::Point::point_values::{FieldEntry, PointFieldType, PointValues};
use napi::bindgen_prelude::Either5;
use napi_derive::napi;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg_attr(not(feature = "native"), napi)]
pub struct Point {
  values: PointValues,
}

#[cfg_attr(not(feature = "native"), napi)]
impl Point {

  #[cfg_attr(not(feature = "native"), napi(constructor))]
  pub fn new(measurement: String) -> Self {
    let values = PointValues::from_measurement(measurement);

    Self { values }
  }

  #[cfg_attr(not(feature = "native"), napi(factory))]
  pub fn from_measurement(measurement: String) -> Self {
    let values = PointValues::from_measurement(measurement);

    Self { values }
  }

  #[cfg_attr(not(feature = "native"), napi(factory))]
  pub fn from_values(values: PointValues) -> Self {
    Self { values }
  }

  #[cfg_attr(not(feature = "native"), napi(getter))]
  pub fn measurement(&self) -> Option<String> {
    self.values.measurement()
  }

  #[cfg_attr(not(feature = "native"), napi(setter, js_name = "measurement"))]
  pub fn set_measurement(&mut self, measurement: String) {
    if !measurement.is_empty() {
      self.values.name = Some(measurement);
    }
  }

  #[cfg_attr(not(feature = "native"), napi(getter))]
  pub fn timestamp(&self) -> Option<u32> {
    self.values.timestamp()
  }

  #[cfg_attr(not(feature = "native"), napi(setter))]
  pub fn set_timestamp(&mut self, timestamp: u32) {
    self.values.set_timestamp(timestamp);
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_tag(&mut self, tag_name: String) {
    self.values.get_tag(tag_name);
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_tag(&mut self, name: String, value: String) {
    self.values.set_tag(name, value);
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn remove_tag(&mut self, name: String) {
    self.values.remove_tag(name);
  }

  #[cfg_attr(not(feature = "native"), napi(getter))]
  pub fn tag_names(&self) -> Vec<String> {
    self.values.tag_names()
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_float_field(&self, name: String) -> napi::Result<Option<f64>> {
    self.values.get_float_field(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_float_field(&mut self, name: String, value: f64) {
    self.values.set_float_field(name, value)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_int_field(&self, name: String) -> napi::Result<Option<i64>> {
    self.values.get_int_field(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_int_field(&mut self, name: String, value: i64) {
    self.values.set_int_field(name, value)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_uinteger_field(&self, name: String) -> napi::Result<Option<u32>> {
    self.values.get_uinteger_field(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_uinteger_field(&mut self, name: String, value: u32) {
    self.values.set_uinteger_field(name, value)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_string_field(&self, name: String) -> napi::Result<Option<String>> {
    self.values.get_string_field(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_string_field(&mut self, name: String, value: String) {
    self.values.set_string_field(name, value)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_boolean_field(&self, name: String) -> napi::Result<Option<bool>> {
    self.values.get_boolean_field(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_boolean_field(&mut self, name: String, value: bool) {
    self.values.set_boolean_field(name, value)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_field_type(&mut self, name: String) -> Option<PointFieldType> {
    self.values.get_field_type(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn set_fields(
    &mut self,
    values: HashMap<String, Either5<bool, f64, u32, i64, String>>,
  ) -> napi::Result<()> {
    self.values.set_fields(values)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn remove_field(&mut self, name: String) -> napi::Result<()> {
    self.values.remove_field(name)
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn get_field_names(&self) -> Vec<String> {
    self.values.get_field_names()
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn has_fields(&self) -> bool {
    self.values.has_fields()
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn to_line_protocol(
    &self,
    time_precision: Option<TimeUnitV2>,
    default_tags: Option<HashMap<String, String>>,
  ) -> Option<String> {
    if self.values.measurement().is_none() {
      return None;
    };

    let tag_escaper = Escaper::escape_tag();
    let mut fields_line = String::new();
    //  Sort method omitted here, bc of BTreeMap
    for (field_name, field_entry) in self.values.get_fields() {
      let line_protocol_value = field_to_line_protocol_string(field_entry);
      if !fields_line.is_empty() {
        fields_line.push(',')
      };

      fields_line.push_str(
        format!(
          "{}={}",
          tag_escaper.escape(field_name),
          &line_protocol_value
        )
        .as_str(),
      );
    }

    if fields_line.is_empty() {
      return None;
    }

    let mut tags_line = String::new();

    let mut tag_names = self.values.tag_names();

    if let Some(default_tags) = default_tags {
      let tag_names_set: HashSet<String> = tag_names.clone().into_iter().collect();

      let mut default_names: Vec<String> = default_tags.keys().cloned().collect();

      default_names.retain(|name| !tag_names_set.contains(name));

      default_names.sort();
      for name in &default_names {
        if !name.is_empty() {
          if let Some(val) = default_tags.get(name) {
            tags_line.push(',');
            tags_line.push_str(format!("{}={}", tag_escaper.escape(name), &val).as_str());
          }
        }
      }
    }

    tag_names.sort();

    tag_names.into_iter().for_each(|name| {
      if !name.is_empty() {
        let value = self.values.get_tag(name.clone());
        if let Some(val) = value {
          tags_line.push(',');
          tags_line.push_str(format!("{}={}", tag_escaper.escape(&name), &val).as_str());
        }
      }
    });

    let time = if let Some(timestamp) = self.values.get_timestamp() {
      timestamp.to_string()
    } else {
      let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
      if let Some(precision) = time_precision {
        match precision {
          TimeUnitV2::Microsecond => now.as_micros().to_string(),
          TimeUnitV2::Millisecond => now.as_millis().to_string(),
          TimeUnitV2::Second => now.as_secs().to_string(),
          TimeUnitV2::Nanosecond => now.as_nanos().to_string(),
        }
      } else {
        now.as_millis().to_string()
      }
    };

    let measurement_escaper = Escaper::escape_measurement();

    Some(format!(
      "{}{} {} {}",
      measurement_escaper.escape(self.values.measurement().unwrap().as_str()),
      &tags_line,
      &fields_line,
      &time
    ))
  }

  #[cfg_attr(not(feature = "native"), napi)]
  pub fn to_string(&self) -> String {
    self.to_line_protocol(None, None).unwrap_or_default()
  }
}

pub fn field_to_line_protocol_string(field: &FieldEntry) -> String {
  match field {
    FieldEntry::Integer(.., i_value) => format!("{}i", i_value),
    FieldEntry::Float(.., f_value) => f_value.to_string(),
    FieldEntry::Boolean(.., b_value) => {
      if *b_value {
        String::from("T")
      } else {
        String::from("F")
      }
    }
    FieldEntry::String(.., s_value) => Escaper::escape_quoted().escape(s_value),
    FieldEntry::UInteger(.., u_value) => format!("{}u", u_value),
  }
}

// #[cfg_attr(not(feature = "native"), napi)]
// impl From<String> for Point {
//     fn from(measurement: String) -> Self {
//         Self { measurement }
//     }
// }
//
// #[cfg_attr(not(feature = "native"), napi)]
// impl From<PointValues> for Point {
//     fn from(point_values: PointValues) -> Self {
//         Self { measurement, values: point_values }
//     }
// }

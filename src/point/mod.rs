mod escape;
pub mod point_values;

use crate::client::options::TimeUnitV2;
use crate::point::escape::{escape, COMMA_EQ_SPACE, COMMA_SPACE};
use crate::point::point_values::{PointFieldType, PointValues};
use napi::bindgen_prelude::Either5;
use napi_derive::napi;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
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

  // #[cfg_attr(not(feature = "native"), napi(factory))]
  // pub fn from_values(values: PointValues) -> Self {
  //   Self { values }
  // }

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
    self.values.measurement()?;

    // if self.values.measurement().is_none() {
    //   return None;
    // };

    let mut fields_line = String::new();
    //  Sort method omitted here, bc of BTreeMap
    for (field_name, field_entry) in self.values.get_fields() {
      if !fields_line.is_empty() {
        fields_line.push(',')
      };

      fields_line.push_str(
        format!(
          "{}={}",
          escape(field_name, COMMA_EQ_SPACE),
          // escape(&line_protocol_value, COMMA_EQ_SPACE)
          &field_entry
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

      for name in &default_names {
        if !name.is_empty() {
          if let Some(val) = default_tags.get(name) {
            tags_line.push(',');
            tags_line.push_str(
              format!(
                "{}={}",
                escape(name, COMMA_EQ_SPACE),
                escape(val, COMMA_EQ_SPACE)
              )
              .as_str(),
            );
          }
        }
      }
    }

    tag_names.into_iter().for_each(|name| {
      if !name.is_empty() {
        let value = self.values.get_tag(name.clone());
        if let Some(val) = value {
          tags_line.push(',');
          tags_line.push_str(format!("{}={}", escape(&name, COMMA_EQ_SPACE), &val).as_str());
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
        now.as_nanos().to_string()
      }
    };

    let result = format!(
      "{}{} {} {}",
      escape(self.values.measurement()?.as_str(), COMMA_SPACE),
      &tags_line,
      &fields_line,
      &time
    );
    Some(result)
  }
}

#[napi]
impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self.to_line_protocol(None, None).unwrap_or_default()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::point::escape::{escape, DOUBLE_QUOTE};

  #[test]
  fn test_string_escape() {
    assert_eq!(
      format!("\"{}\"", escape(r#"foo"#, DOUBLE_QUOTE)),
      r#""foo""#
    );
    assert_eq!(
      format!("\"{}\"", escape(r"foo \ bar", DOUBLE_QUOTE)),
      r#""foo \\ bar""#
    );
    assert_eq!(
      format!("\"{}\"", escape(r#"foo " bar"#, DOUBLE_QUOTE)),
      r#""foo \" bar""#
    );
    assert_eq!(
      format!("\"{}\"", escape(r#"foo \" bar"#, DOUBLE_QUOTE)),
      r#""foo \\\" bar""#
    );
  }

  #[test]
  fn test_lp_builder() {
    const PLAIN: &str = "plain";
    const WITH_SPACE: &str = "with space";
    const WITH_COMMA: &str = "with,comma";
    const WITH_EQ: &str = "with=eq";
    const WITH_DOUBLE_QUOTE: &str = r#"with"doublequote"#;
    const WITH_SINGLE_QUOTE: &str = "with'singlequote";
    const WITH_BACKSLASH: &str = r"with\ backslash";

    let mut line_one = Point::from_measurement("tag_keys".to_string());

    line_one.set_tag(PLAIN.to_string(), "dummy".to_string());
    line_one.set_tag(WITH_SPACE.to_string(), "dummy".to_string());
    line_one.set_tag(WITH_COMMA.to_string(), "dummy".to_string());
    line_one.set_tag(WITH_EQ.to_string(), "dummy".to_string());
    line_one.set_tag(WITH_DOUBLE_QUOTE.to_string(), "dummy".to_string());
    line_one.set_tag(WITH_SINGLE_QUOTE.to_string(), "dummy".to_string());
    line_one.set_tag(WITH_BACKSLASH.to_string(), "dummy".to_string());
    line_one.set_boolean_field("dummy".to_string(), true);

    let mut line_two = Point::from_measurement("tag_values".to_string());

    line_two.set_tag("plain".to_string(), PLAIN.to_string());
    line_two.set_tag("withspace".to_string(), WITH_SPACE.to_string());
    line_two.set_tag("withcomma".to_string(), WITH_COMMA.to_string());
    line_two.set_tag("witheq".to_string(), WITH_EQ.to_string());
    line_two.set_tag("withdoublequote".to_string(), WITH_DOUBLE_QUOTE.to_string());
    line_two.set_tag("withsinglaquote".to_string(), WITH_SINGLE_QUOTE.to_string());
    line_two.set_tag("withbackslash".to_string(), WITH_BACKSLASH.to_string());
    line_two.set_boolean_field("dummy".to_string(), true);

    let mut line_three = Point::from_measurement("field keys".to_string());

    line_three.set_boolean_field(PLAIN.to_string(), true);
    line_three.set_boolean_field(WITH_SPACE.to_string(), true);
    line_three.set_boolean_field(WITH_COMMA.to_string(), true);
    line_three.set_boolean_field(WITH_EQ.to_string(), true);
    line_three.set_boolean_field(WITH_DOUBLE_QUOTE.to_string(), true);
    line_three.set_boolean_field(WITH_SINGLE_QUOTE.to_string(), true);
    line_three.set_boolean_field(WITH_BACKSLASH.to_string(), true);

    let mut line_four = Point::from_measurement("field values".to_string());

    line_four.set_boolean_field("mybool".to_string(), false);
    line_four.set_int_field("mysigned".to_string(), 51_i64);
    line_four.set_uinteger_field("myunsigned".to_string(), 51_u32);
    line_four.set_float_field("myfloat".to_string(), 51.0);
    line_four.set_string_field("mystring".to_string(), "some value".to_string());
    line_four.set_string_field(
      "mystringwithquotes".to_string(),
      "some \" value".to_string(),
    );

    let mut line_five = Point::from_measurement(PLAIN.to_string());
    line_five.set_boolean_field("dummy".to_string(), true);

    let mut line_six = Point::from_measurement(WITH_SPACE.to_string());
    line_six.set_boolean_field("dummy".to_string(), true);

    let mut line_seven = Point::from_measurement(WITH_COMMA.to_string());
    line_seven.set_boolean_field("dummy".to_string(), true);

    let mut line_eight = Point::from_measurement(WITH_EQ.to_string());
    line_eight.set_boolean_field("dummy".to_string(), true);

    let mut line_nine = Point::from_measurement(WITH_DOUBLE_QUOTE.to_string());
    line_nine.set_boolean_field("dummy".to_string(), true);

    let mut line_ten = Point::from_measurement(WITH_SINGLE_QUOTE.to_string());
    line_ten.set_boolean_field("dummy".to_string(), true);

    let mut line_eleven = Point::from_measurement(WITH_BACKSLASH.to_string());
    line_eleven.set_boolean_field("dummy".to_string(), true);

    let mut line_twelve = Point::from_measurement("without timestamp".to_string());
    line_twelve.set_boolean_field("dummy".to_string(), true);

    let mut line_thirteen = Point::from_measurement("with timestamp".to_string());
    line_thirteen.set_boolean_field("dummy".to_string(), true);
    line_thirteen.set_timestamp(1234);

    let lines: [Point; 13] = [
      line_one,
      line_two,
      line_three,
      line_four,
      line_five,
      line_six,
      line_seven,
      line_eight,
      line_nine,
      line_ten,
      line_eleven,
      line_twelve,
      line_thirteen,
    ];
    let lp: Vec<String> = lines
      .into_iter()
      .map(|l| l.to_line_protocol(None, None).unwrap())
      .collect();
    println!("-----\n{}-----", lp.join("\n"));
  }
}

use influxdb3_napi::client::options::{Precision, QueryPayload, TimeUnitV2};
use influxdb3_napi::client::{InfluxDBClient, WriteOptions};
use influxdb3_napi::point::Point;
use influxdb3_napi::serializer::Serializer;

#[tokio::test]
async fn test_write_points_cloud_serverless() {
  dotenvy::dotenv().ok();
  let server_addr = std::env::var("SERVER_URL").expect("MY_SECRET_KEY not set in .env");
  let token = std::env::var("API_TOKEN").expect("MY_SECRET_KEY not set in .env");

  let mut write_options = WriteOptions::default();
  write_options.no_sync = Some(false);
  write_options.precision = Some(Precision::V2(TimeUnitV2::Nanosecond));

  let mut client = InfluxDBClient::new(server_addr, Some(token), Some(Serializer::Library), None);

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

  let lp = line_one.to_line_protocol(None, None).unwrap();

  println!("wtf {}", lp);
  let result = client
    .write(vec![lp], String::from("test"), Some(write_options), None)
    .await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn test_read_data_cloud_serverless() {
  dotenvy::dotenv().ok();
  let server_addr = std::env::var("SERVER_URL").expect("MY_SECRET_KEY not set in .env");
  let token = std::env::var("API_TOKEN").expect("MY_SECRET_KEY not set in .env");

  let mut client = InfluxDBClient::new(server_addr, Some(token), Some(Serializer::Library), None);

  let result = client
    .query(QueryPayload {
      database: "test".to_string(),
      query: r#"SELECT * FROM "tag_keys" "#.to_string(),
      _type: None,
      params: None,
    })
    .await;

  assert!(result.is_ok());
}

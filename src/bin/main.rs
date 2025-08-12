use influxdb3_napi::client::options::{QueryPayload, QueryType};
use napi::bindgen_prelude::Either3;
use napi::tokio;
use napi::tokio_stream::StreamExt;

#[tokio::main]
#[cfg(feature = "native")]
async fn main() {
  // std::env::set_var("RUST_LOG", "hyper=trace,tonic=trace,h2=trace");
  // tracing_subscriber::fmt::init();
  //
  //
  //
  //
  // let mut client = influxdb3_napi::client::InfluxDBClient::new(
  //       // String::from("http://165.232.154.186:8195"),
  //       // Some(String::from("apiv3_64thndtOkGj3gpj5Mc3IwgSN9jKJ6c2Jle4-sJdQwsZ5nIThBjT9ALB0GjEpXvSgt2ZotiQzLbdtbFTEi8S2hg")),
  //       String::from("http://165.232.154.186:8192"),
  //       Some(String::from("apiv3_3UbvWHEjw_1Jo9qaz0DrH0g59mA_o6KxsDb8WTZI5Gd5PvzTOvD_UDocfMZ1yqnyvJmBd2Wrkkx8MXfVgcpDew")),
  //       Some(influxdb3_napi::serializer::Serializer::Library),
  //       None
  //   );
  //
  // let mut response = client
  //   .query(QueryPayload {
  //     database: "sample-air-sensor-1754383690707".to_string(),
  //     query: "SELECT * FROM system.last_caches".to_string(),
  //     _type: Some(QueryType::Sql),
  //     params: None,
  //   })
  //   .await
  //   .unwrap();
  //
  // match response {
  //   Either3::A(mut data) => match data.next().await {
  //     Some(Ok(res)) => {
  //       println!("result is {:?}", res)
  //     }
  //     _ => println!("no result found"),
  //   },
  //   _ => panic!(),
  // }
}

#[tokio::main]
#[cfg(not(feature = "native"))]
async fn main() {
  println!("Not implemented yet");
}

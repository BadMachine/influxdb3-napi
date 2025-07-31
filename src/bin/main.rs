use std::collections::HashMap;
use napi::bindgen_prelude::Either5;
use napi::tokio;
use napi::tokio::time::Instant;
use influxdb_client::client::{WriteOptions, QueryType};
use influxdb_client::point::Point;

#[tokio::main]
#[cfg(feature = "native")]
async fn main(){
    let mut client = influxdb_client::client::InfluxDBClient::new(
        // String::from("http://165.232.154.186:8195"),
        // Some(String::from("apiv3_64thndtOkGj3gpj5Mc3IwgSN9jKJ6c2Jle4-sJdQwsZ5nIThBjT9ALB0GjEpXvSgt2ZotiQzLbdtbFTEi8S2hg")),
        String::from("http://165.232.154.186:8184"),
        Some(String::from("apiv3_KvLXBM04k9Ly83GI1ZLbNNaP0Ilh6DfkAN7fIclBTAK2eKG3WlgE4q8lZvlVpiHgR-a7GZntmZvolV1f2Lc6iQ")),
        Some(influxdb_client::serializer::Serializer::Library),
        None
    );

    let start = Instant::now();

    let mut response = client.query_batch(String::from("_internal"), String::from("SELECT * FROM system.databases"), Some(QueryType::Sql)).await .unwrap();
    //

    match response.next().await {
        Ok(res) => {
            match res {
                Some(data) => {
                    println!("result is {:?}", data);
                },
                None => {
                    println!("Empty response");
                }
            }
        },
        Err(e) => {
            println!("Error occurred while the request {}", e);
        }
    }


    EscapedStr

    let mut point = Point::from_measurement(String::from("test_measurement"));

    let mut fields: HashMap<String, Either5<bool, f64, u32, i64, String>> = HashMap::new();
    fields.insert(String::from("A"), Either5::C(4));
    fields.insert(String::from("B"), Either5::B(3.));

    point.set_fields(fields).expect("TODO: panic message");


    println!("Full elapsed time: {:?}", start.elapsed());


    println!("Line protocol: {:?}", point.to_line_protocol(None, None));

    let mut write_options = WriteOptions::default();
    write_options.no_sync = Some(true);

    // client.write(vec![String::from("cpu,host=server01 usage=85.2 1638360000000")], String::from("test"), Some(write_options), None).await;
    client.write(vec![point.to_line_protocol(None, None).unwrap()], String::from("test_line"), Some(write_options), None).await;


}

#[tokio::main]
#[cfg(not(feature = "native"))]
async fn main(){
    println!("Not compiled yet");

}
use std::collections::HashMap;
use napi::bindgen_prelude::Either5;
use napi::tokio;
use napi::tokio::time::Instant;
use influxdb_client::client::QueryType;
use influxdb_client::point::Point;

#[tokio::main]
#[cfg(feature = "native")]
async fn main(){
    let mut client = influxdb_client::client::InfluxDBClient::new(
        // String::from("http://165.232.154.186:8195"),
        // Some(String::from("apiv3_64thndtOkGj3gpj5Mc3IwgSN9jKJ6c2Jle4-sJdQwsZ5nIThBjT9ALB0GjEpXvSgt2ZotiQzLbdtbFTEi8S2hg")),
        String::from("https://us-east-1-1.aws.cloud2.influxdata.com"),
        Some(String::from("8D4v3HCeOqrrw7dTZa2PZy34484BusR8rDNTKEzMRbQR2ETV-P7YShtS4clGdI3PTai5nG5m8gc5abRihppQuA==")),
        Some(influxdb_client::serializer::Serializer::Library),
        None
    );

    let start = Instant::now();

    let mut response = client.query_batch(String::from("GF HA Data"), String::from("SELECT * FROM car_mileage"), QueryType::Sql).await .unwrap();
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

    let mut point = Point::from_measurement(String::from("test"));

    let mut fields: HashMap<String, Either5<bool, f64, u32, i64, String>> = HashMap::new();
    fields.insert(String::from("A"), Either5::E(String::from("test")));
    fields.insert(String::from("B"), Either5::A(false));

    point.set_fields(fields).expect("TODO: panic message");


    println!("Full elapsed time: {:?}", start.elapsed());


    println!("Line protocol: {:?}", point.to_line_protocol(None, None));

    client.write(vec![point.to_line_protocol(None, None).expect("REASON")], String::from("GF HA Data"), None, None).await;

    // client.write().await;
}

#[tokio::main]
#[cfg(not(feature = "native"))]
async fn main(){
    println!("Not compiled yet");

}
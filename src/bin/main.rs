use napi::tokio;
use tokio::time::Instant;
use influxdb_client::client::QueryType;

#[tokio::main]
#[cfg(feature = "native")]
async fn main(){
    let mut client = influxdb_client::client::InfluxDBClient::new(
        String::from("http://165.232.154.186:8195"),
        Some(String::from("apiv3_64thndtOkGj3gpj5Mc3IwgSN9jKJ6c2Jle4-sJdQwsZ5nIThBjT9ALB0GjEpXvSgt2ZotiQzLbdtbFTEi8S2hg")),
        Some(influxdb_client::serializer::Serializer::Library),
        None
    );

    let start = Instant::now();

    let mut response = client.query_batch(String::from("_internal"), String::from("SELECT * from system.queries WHERE query_text IS NOT NULL LIMIT 2"), QueryType::Sql).await .unwrap();
    //
    if let Ok(res) = response.next().await {
        match res {
            Some(data) => {
                println!("{:?}", data);
            },
            None => {}
        }
    }

    println!("Elapsed time: {:?}", start.elapsed());

    // client.write().await;
}

#[tokio::main]
#[cfg(not(feature = "native"))]
async fn main(){

}
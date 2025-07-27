#[tokio::main]
#[cfg(feature = "native")]
async fn main(){
    let mut client = influxdb_client::client::InfluxDBClient::new(
        String::from("http://165.232.154.186:8197"),
        Some(String::from("apiv3_dWdcY2ZEkmTTnc9K2Hc69UBOJsbWF9DGxN1jEHYTyuv3jcYWBT_XibVKnDyNcr_StU2jSwOdvZaEbsTc-11wpA")),
        Some(influxdb_client::serializer::Serializer::Library)
    );

    let mut response = client.query_batch(String::from("sample-bird-migration-1753351273642"), String::from("SHOW FIELD KEYS"), influxdb_client::client::QueryType::Influxql).await.unwrap();

    if let Ok(res) = response.next().await {
        match res {
            Some(data) => {
                println!("{:?}", data);
            },
            None => {
                println!("---NO DATA---");
            }
        }
    } else {
        println!("Error occured :(");
    }
}

#[tokio::main]
#[cfg(not(feature = "native"))]
async fn main(){

}
use influxdb_client::client::QueryType;

#[tokio::main]
async fn main(){
    // let mut client = influxdb_client::client::InfluxDBClient::new(
    //     String::from("http://165.232.154.186:8197"),
    //     Some(String::from("apiv3_dWdcY2ZEkmTTnc9K2Hc69UBOJsbWF9DGxN1jEHYTyuv3jcYWBT_XibVKnDyNcr_StU2jSwOdvZaEbsTc-11wpA")),
    // );
    //
    // let mut response = client.query_batch(String::from("_internal"), String::from("SELECT * from system.queries WHERE query_text IS NOT NULL LIMIT 1"), QueryType::Sql).await .unwrap();

    // if let Ok(res) = response.next().await {
    //     match res {
    //         Some(data) => {
    //             // println!("{:?}", data);
    //         },
    //         None => {}
    //     }
    // }
}
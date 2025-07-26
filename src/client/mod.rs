use crate::query::by_batch::QueryResultByBatch;

use std::fmt;

use arrow_flight::{FlightClient, Ticket};
use napi::bindgen_prelude::block_on;
use tonic::codegen::Bytes;
use tonic::transport::Channel;


#[napi_derive::napi(string_enum)]
pub enum QueryType {
  #[napi(value = "sql")]
  Sql,
  #[napi(value = "flux")]
  Flux,
}

impl fmt::Display for QueryType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      QueryType::Sql => write!(f, "sql"),
      QueryType::Flux => write!(f, "flux"),
    }
  }
}

#[cfg_attr(feature = "napi", napi_derive::napi)]
pub struct InfluxDBClient {
  flight_client: FlightClient,
}

#[cfg_attr(feature = "napi", napi_derive::napi)]
impl InfluxDBClient {
  #[cfg_attr(feature = "napi", napi_derive::napi(constructor))]
  pub fn new(addr: String, token: Option<String>) -> Self {
    #[cfg(feature = "napi")]
    let channel = block_on(async {
      Channel::from_shared(addr)
        .unwrap()
        .connect()
        .await
        .expect("error connecting")
    });
    #[cfg(not(feature = "napi"))]
    let channel = Channel::from_shared(addr).unwrap().connect_lazy();

    let mut flight_client = FlightClient::new(channel);
    if let Some(token) = token {
      flight_client
        .add_header("authorization", format!("Bearer {}", token).as_str())
        .unwrap();
    }

    Self { flight_client }
  }

  #[cfg(feature = "napi")]
  #[cfg_attr(feature = "napi", napi_derive::napi)]
  pub async unsafe fn query_batch(
    &mut self,
    database: String,
    query: String,
    _type: QueryType,
  ) -> Result<QueryResultByBatch, napi::Error> {
    let payload = format!(
      "{{ \"database\": \"{}\", \"sql_query\": \"{}\", \"query_type\": \"{}\" }}",
      database, query, _type
    )
    .replace("\n", " ");

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    let response = self.flight_client.do_get(ticket).await.unwrap();

    let result = QueryResultByBatch::new(response);

    Ok(result)
  }

  #[cfg(not(feature = "napi"))]
  pub async fn query_batch(
    &mut self,
    database: String,
    query: String,
    _type: QueryType,
  ) -> Result<QueryResultByBatch, napi::Error> {
    let payload = format!(
      "{{ \"database\": \"{}\", \"sql_query\": \"{}\", \"query_type\": \"{}\" }}",
      database, query, _type
    )
    .replace("\n", " ");

    let ticket = Ticket {
      ticket: Bytes::from(payload),
    };

    let response = self.flight_client.do_get(ticket).await.unwrap();

    let result = QueryResultByBatch::new(response);

    Ok(result)
  }
}

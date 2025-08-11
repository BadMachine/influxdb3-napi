use crate::client::options::FlightOptions;
use std::time::Duration;
use tonic::transport::{Channel, Endpoint};

pub fn get_channel(addr: String, flight_options: Option<FlightOptions>) -> Endpoint {
  let opts = flight_options.unwrap_or_default();

  let keep_alive_interval = opts.keep_alive_interval.unwrap_or_default().into();

  let keep_alive_timeout = opts.keep_alive_timeout.unwrap_or_default().into();

  Channel::from_shared(addr)
    .unwrap()
    .keep_alive_while_idle(true)
    .http2_keep_alive_interval(Duration::from_secs(keep_alive_interval))
    .keep_alive_timeout(Duration::from_secs(keep_alive_timeout))
    .tls_config(tonic::transport::ClientTlsConfig::new().with_webpki_roots())
    .unwrap()
}

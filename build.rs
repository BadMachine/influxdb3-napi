// use prost_build::Config;

fn main() {
  // let mut config = Config::default();

// config.type_attribute(".influxdata.iox.querier.v1.ReadInfo.QueryType", "#[napi_derive::napi]");
// config.field_attribute(".influxdata.iox.querier.v1.ReadInfo.QueryType.QUERY_TYPE_UNSPECIFIED", r#"#[napi]"#);
// config.field_attribute(".influxdata.iox.querier.v1.ReadInfo.QueryType.QUERY_TYPE_SQL", r#"#[napi(value = "sql")]"#);
// config.field_attribute(".influxdata.iox.querier.v1.ReadInfo.QueryType.QUERY_TYPE_INFLUX_QL", r#"#[napi(value = "influxql")]"#);
// config.field_attribute(".influxdata.iox.querier.v1.ReadInfo.QueryType.QUERY_TYPE_FLIGHT_SQL_MESSAGE", r#"#[napi(value = "flight_sql_message")]"#);

// config.type_attribute(".rustplus.Member", "#[serde_with::serde_as]");
// config.type_attribute(".rustplus.AppTeamMessage", "#[serde_with::serde_as]");
// config.type_attribute(".rustplus.AppMarker", "#[serde_with::serde_as]");
//
// config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize, specta::Type)]");
// config.field_attribute(".rustplus.AppTeamInfo.leaderSteamId", r#"#[serde_as(as = "serde_with::DisplayFromStr")]"#);
// config.field_attribute(".rustplus.AppTeamInfo.Member.steamId", r#"#[serde_as(as = "serde_with::DisplayFromStr")]"#);
// config.field_attribute(".rustplus.AppTeamMessage.steamId", r#"#[serde_as(as = "serde_with::DisplayFromStr")]"#);
//
// // config.field_attribute(".rustplus.AppMarker.steamId", r#"#[serde_as(as = "Option<serde_with::DisplayFromStr>")]"#);
// config.field_attribute(".rustplus.AppMarker.steamId", r#"#[serde_as(as = "serde_with::DisplayFromStr")]"#);

  // config
  //     .compile_protos(
  //       &["src/proto/flight.proto"],
  //       &["src/proto/"],
  //     )
  //     .unwrap();

  napi_build::setup();
}

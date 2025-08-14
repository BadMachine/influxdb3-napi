pub struct UnsafeSerializer;

impl UnsafeSerializer {
  pub(crate) async fn serialize(columns: &Vec<String>, values: &Vec<serde_json::Value>) -> Option<serde_json::Map<String, serde_json::Value>> {
    let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    for (col, val) in columns.iter().zip(values.iter()) {
      map.insert((*col).to_string(), val.clone());
    }

    Some(map)
  }
}

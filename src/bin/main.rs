use napi::tokio;

#[tokio::main]
#[cfg(feature = "native")]
async fn main() {
  println!("Not implemented yet for native binary");
}

#[tokio::main]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "native"))]
async fn main() {
  println!("Not implemented yet");
}

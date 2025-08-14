#[cfg(not(target_arch = "wasm32"))]
pub mod channel;

#[cfg(feature = "native")]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod browser;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "native"))]
pub mod napi_rs;

pub mod http_client;
pub mod options;

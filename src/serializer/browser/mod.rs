#[cfg(target_arch = "wasm32")]
pub(crate) mod unsafe_serializer;

#[napi_derive::napi]
pub enum Serializer {
    Unsafe
}
impl Default for Serializer {
    fn default() -> Self {
        Self::Unsafe
    }
}

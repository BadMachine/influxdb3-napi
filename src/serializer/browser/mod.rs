#[cfg(target_arch = "wasm32")]
pub(crate) mod unsafe_serializer;

pub enum Serializer {
    Unsafe
}
impl Default for Serializer {
    fn default() -> Self {
        Self::Unsafe
    }
}

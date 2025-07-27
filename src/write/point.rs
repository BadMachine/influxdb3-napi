
#[cfg_attr(not(feature = "native"), napi_derive::napi)]
pub struct Point {
    measurement: String,
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl Point {
    #[cfg_attr(not(feature = "native"), napi_derive::napi(constructor))]
    pub fn new(measurement: String) -> Self {
        Self { measurement }
    }
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl From<String> for Point {
    fn from(measurement: String) -> Self {
        Self { measurement }
    }
}
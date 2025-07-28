use napi_derive::napi;

#[cfg_attr(not(feature = "native"), napi)]
pub struct Point {
    measurement: String,
}

#[cfg_attr(not(feature = "native"), napi)]
impl Point {
    #[cfg_attr(not(feature = "native"), napi(constructor))]
    pub fn new(measurement: String) -> Self {
        Self { measurement }
    }
}

#[cfg_attr(not(feature = "native"), napi)]
impl From<String> for Point {
    fn from(measurement: String) -> Self {
        Self { measurement }
    }
}
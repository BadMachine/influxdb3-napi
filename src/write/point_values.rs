use std::collections::HashMap;

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
pub struct PointValues {
    name: Option<String>,
    time: Option<u32>,
    tags: HashMap<String, String>,
    fields: HashMap<String, String>,
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl PointValues {
    #[cfg_attr(not(feature = "native"), napi_derive::napi(constructor))]
    pub fn new() -> Self {
        Self {
            name: None,
            time: None,
            tags: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    #[cfg_attr(not(feature = "native"), napi_derive::napi(getter))]
    pub fn measurement(&self) -> Option<String> {
        self.name.clone()
    }

    #[cfg_attr(not(feature = "native"), napi_derive::napi(getter))]
    pub fn timestamp(&self) -> Option<u32> {
        self.time
    }

    #[cfg_attr(not(feature = "native"), napi_derive::napi(setter))]
    #[cfg_attr(not(feature = "native"), napi_derive::napi(js_name = "timestamp"))]
    pub fn set_timestamp(&mut self, time: u32) -> &mut PointValues {
        self.time = Some(time);
        self
    }

    #[cfg_attr(not(feature = "native"), napi_derive::napi)]
    pub fn get_tag(&self, tag_name: String) -> Option<String> {
        self.tags.get(&tag_name).cloned()
    }

    // #[cfg_attr(not(feature = "native"), napi_derive::napi)]
    // pub fn set_tag(&mut self, tag_name: String, tag_value: String) -> &mut PointValues {
    //     self.tags.insert(tag_name, tag_value);
    //     self
    // }
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
impl Default for PointValues {
    fn default() -> Self {
        Self {
            name: None,
            time: None,
            tags: HashMap::new(),
            fields: HashMap::new(),
        }
    }
}

use std::collections::HashMap;
use napi_derive::napi;

#[cfg_attr(not(feature = "native"), napi)]
pub struct PointValues {
    name: Option<String>,
    time: Option<u32>,
    tags: HashMap<String, String>,
    fields: HashMap<String, String>,
}

#[cfg_attr(not(feature = "native"), napi)]
impl PointValues {

    #[cfg_attr(not(feature = "native"), napi(getter))]
    pub fn measurement(&self) -> Option<String> {
        self.name.clone()
    }

    #[cfg_attr(not(feature = "native"), napi(getter))]
    pub fn timestamp(&self) -> Option<u32> {
        self.time
    }


    #[cfg_attr(not(feature = "native"), napi_derive::napi)]
    pub fn set_timestamp(&mut self, time: u32) -> Self {
        let name = self.name.clone();
        let tags = self.tags.clone();
        let fields = self.fields.clone();

        Self {
            name,
            time: Some(time),
            tags,
            fields,
        }
    }

    #[cfg_attr(not(feature = "native"), napi)]
    pub fn get_tag(&self, tag_name: String) -> Option<String> {
        self.tags.get(&tag_name).cloned()
    }

    #[cfg_attr(not(feature = "native"), napi_derive::napi)]
    pub fn set_tag(&mut self, tag_name: String, tag_value: String) -> Self {
        let mut tags = self.tags.clone();
        tags.insert(tag_name, tag_value);

        let fields = self.fields.clone();

        let name = self.name.clone();

        Self {
            name,
            time: self.time,
            tags,
            fields,
        }
    }
}

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

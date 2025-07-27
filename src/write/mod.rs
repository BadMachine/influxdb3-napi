pub mod point;
mod point_values;

use std::collections::HashMap;
use napi_derive::napi;

#[napi_derive::napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnit {
    /// Time in seconds.
    #[napi(value = "s")]
    Second,
    /// Time in milliseconds.
    #[napi(value = "ms")]
    Millisecond,
    /// Time in microseconds.
    #[napi(value = "us")]
    Microsecond,
    /// Time in nanoseconds.
    #[napi(value = "ns")]
    Nanosecond,
}

#[cfg_attr(not(feature = "native"), napi_derive::napi(object))]
pub struct WriteOptions {
    /** Precision to use in writes for timestamp. default ns */
    pub precision: Option<TimeUnit>,
    /** HTTP headers that will be sent with every write request */
    //headers?: {[key: string]: string}
    pub headers: Option<HashMap<String, String>>,
    /** When specified, write bodies larger than the threshold are gzipped  */
    pub gzip_threshold: u32,
    /**
    * Instructs the server whether to wait with the response until WAL persistence completes.
    * noSync=true means faster write but without the confirmation that the data was persisted.
    *
    * Note: This option is supported by InfluxDB 3 Core and Enterprise servers only.
    * For other InfluxDB 3 server types (InfluxDB Clustered, InfluxDB Clould Serverless/Dedicated)
    * the write operation will fail with an error.
    *
    * Default value: false.
    */
    pub no_sync: Option<bool>,

    pub default_tags: Option<HashMap<String, String>>,
}

#[cfg_attr(not(feature = "native"), napi_derive::napi)]
pub const DEFAULT_WRITE_OPTIONS: WriteOptions = WriteOptions {
    precision: Some(TimeUnit::Nanosecond),
    headers: None,
    gzip_threshold: 1000,
    no_sync: Some(false),
    default_tags: None,
};

// #[cfg_attr(not(feature = "native"), napi_derive::napi)]
// pub fn get_default_write_options() -> WriteOptions {
//     WriteOptions {
//         precision: Some(TimeUnit::Nanosecond),
//         headers: None,
//         gzip_threshold: 1000,
//         no_sync: Some(false),
//         default_tags: None,
//     }
// }
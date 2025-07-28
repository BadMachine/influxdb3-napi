// NAPI DOES NOT SUPPORT GENERIC STUCTURE DEPS

// use std::io::Cursor;
// use arrow::array::RecordBatch;
// use arrow::json::LineDelimitedWriter;
// use napi::bindgen_prelude::ToNapiValue;
// use crate::serializer::Serializer;
//
// #[cfg_attr(feature = "napi", napi_derive::napi)]
// pub struct UnsafeSerializer {}
//
// impl Serializer for UnsafeSerializer {
//     async fn compute(batch: Option<arrow_flight::error::Result<RecordBatch>>) -> napi::Result<Option<impl ToNapiValue>> {
//         if let Some(Ok(batch)) = batch {
//             let mut buf = Vec::new();
//             {
//                 let cursor = Cursor::new(&mut buf);
//                 let mut writer = LineDelimitedWriter::new(cursor);
//                 writer.write(&batch).unwrap();
//                 writer.finish().unwrap();
//             }
//
//             let items: Vec<serde_json::Value> = serde_arrow::from_record_batch(&batch).unwrap();
//
//             Ok(Some(items))
//         } else {
//             Ok(None)
//         }
//     }
// }


use arrow::array::RecordBatch;

pub fn unsafe_serialize(batch: Option<arrow_flight::error::Result<RecordBatch>>) -> Option<Vec<serde_json::Value>> {

    if let Some(Ok(batch)) = batch {

        match serde_arrow::from_record_batch::<Vec<serde_json::Value>>(&batch) {
            Ok(parsed) => {
                Some(parsed)
            },
            _ => None
        }
    } else {
        None
    }
}
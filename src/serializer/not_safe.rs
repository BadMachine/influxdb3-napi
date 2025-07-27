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

        // SIMD implementation didnt show efficient results

        // // Write the record batch out as a JSON array
        // let buf = Vec::new();
        // let mut writer = arrow::json::ArrayWriter::new(buf);
        // writer.write_batches(&vec![&batch]).unwrap();
        // writer.finish().unwrap();
        //
        // let mut buf = writer.into_inner();
        //
        // let st_simd = Instant::now();
        // let v: sonic_rs::Value = sonic_rs::from_slice(&mut buf).unwrap();
        // // let v: simd_json::OwnedValue = simd_json::to_owned_value(&mut buf).unwrap();
        // let end_simd = Instant::now();

        // println!("simd serialization time {:?}", end_simd.duration_since(st_simd).as_millis());

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
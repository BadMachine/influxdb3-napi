use crate::serializer::Serializer;

use arrow_flight::decode::FlightRecordBatchStream;
use napi_derive::napi;

#[derive(Debug)]
#[cfg_attr(not(feature = "native"), napi)]
pub struct QueryResult {
  pub(crate) response: FlightRecordBatchStream,
  pub serializer: Serializer,
}

#[cfg_attr(not(feature = "native"), napi)]
impl QueryResult {
  pub fn new(response: FlightRecordBatchStream, serializer: Option<Serializer>) -> Self {
    Self {
      response,
      serializer: serializer.unwrap_or(Serializer::Unsafe),
    }
  }

  // pub async fn next(
  //   &mut self,
  // ) -> napi::Result<Option<Either<Vec<crate::ReturnDataType>, Vec<serde_json::Value>>>> {
  //   self.next_inner().await
  // }
}


use napi::bindgen_prelude::*;
use napi::bindgen_prelude::async_iterator::AsyncGenerator;

// // Ваша структура
// #[napi]
// pub struct MyAsyncIterator {
//   current: i32,
//   max: i32,
// }
//
// #[napi]
// impl MyAsyncIterator {
//   #[napi(constructor)]
//   pub fn new(max: i32) -> Self {
//     Self { current: 0, max }
//   }
// }
//
// #[napi]
// impl AsyncGenerator for MyAsyncIterator {
//   type Yield = i32;
//   type Next = i32;
//   type Return = i32;
//
//   fn next(
//     &mut self,
//     _value: Option<Self::Next>,
//   ) -> impl Future<Output = napi::Result<Option<Self::Yield>>> + Send + 'static {
//     if self.current < self.max {
//       let result = self.current;
//       self.current += 1;
//       std::future::ready(Ok(Some(result)))
//     } else {
//       std::future::ready(Ok(None))
//     }
//   }
// }

#[napi]
pub struct MyAsyncIterator {
  current: i32,
  max: i32,
}

#[napi]
impl MyAsyncIterator {
  #[napi(constructor)]
  pub fn new(max: i32) -> Self {
    Self { current: 0, max }
  }

  // Метод, который возвращает async iterator
  #[napi(js_name = "[Symbol.asyncIterator]")]
  pub fn async_iterator(&self) -> AsyncIteratorWrapper {
    AsyncIteratorWrapper {
      current: self.current,
      max: self.max,
    }
  }
}

#[napi]
pub struct AsyncIteratorWrapper {
  current: i32,
  max: i32,
}

#[napi]
impl AsyncIteratorWrapper {
  #[napi]
  pub async unsafe fn next(&mut self) -> napi::Result<IteratorResult> {
    if self.current < self.max {
      let value = self.current;
      self.current += 1;
      Ok(IteratorResult {
        value: Some(value),
        done: false,
      })
    } else {
      Ok(IteratorResult {
        value: None,
        done: true,
      })
    }
  }

  #[napi]
  pub async unsafe fn return_(&mut self, _value: Option<i32>) -> napi::Result<IteratorResult> {
    Ok(IteratorResult {
      value: None,
      done: true,
    })
  }

  #[napi]
  pub async unsafe fn throw(&mut self, _error: String) -> napi::Result<IteratorResult> {
    Err(napi::Error::new(
      napi::Status::GenericFailure,
      "Iterator was thrown".to_string(),
    ))
  }
}

#[napi(object)]
pub struct IteratorResult {
  pub value: Option<i32>,
  pub done: bool,
}
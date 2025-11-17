#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn hello() -> i32 {
  0
}

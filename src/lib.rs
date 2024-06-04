#![deny(clippy::all)]

use lightningcss::properties::background::Background;
use lightningcss::traits::Parse;
use lightningcss::values::image;
use serde_json::to_string;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn parse_to_json_str(val: String) -> String {
  let bg = Background::parse_string(val.as_str()).unwrap();
  match bg.image {
    image::Image::Gradient(g) => {
      let gradient = to_string(&g).unwrap();
      gradient
    }
    _ => "Not a gradient".to_string(),
  }
}

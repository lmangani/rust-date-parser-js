extern crate chrono;

use wasm_bindgen::prelude::*;
use chrono::prelude::*;

#[wasm_bindgen]
pub fn parse_rfc3339(s: String) -> String {
   // convert the string into DateTime<FixedOffset>
   let datetime = DateTime::parse_from_rfc3339(&s).unwrap();
   let format = "%s%6f";
   return datetime.format(format).to_string();
}

#[wasm_bindgen]
pub fn now_rfc3339() -> String {
   let local: DateTime<Local> = Local::now();
   return local.to_rfc3339();  
}

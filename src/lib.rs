extern crate chrono;

use wasm_bindgen::prelude::*;
use chrono::prelude::*;
use std::time::SystemTime;

pub fn convert_nanos(timestamp: i64)  -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp / 1000000, (timestamp % 1000000) as u32 * 1_000).unwrap();
    DateTime::<Utc>::from_utc(naive, Utc)
}

pub fn convert_micros(timestamp: i64)  -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp / 1000, (timestamp % 1000) as u32 * 1_000_000).unwrap();
    DateTime::<Utc>::from_utc(naive, Utc)
}

#[wasm_bindgen]
pub fn parse_rfc3339(s: String) -> String {
   // convert the string into DateTime<FixedOffset>
   let datetime = DateTime::parse_from_rfc3339(&s).unwrap();
   let format = "%s%6f";
   return datetime.format(format).to_string();
}

#[wasm_bindgen]
pub fn parse_nanos(s: String) -> String {
    let timestamp = s.parse::<i64>().unwrap();
    let ts = convert_nanos(timestamp);
    return ts.to_rfc3339();
}

#[wasm_bindgen]
pub fn parse_micros(s: String) -> String {
    let timestamp = s.parse::<i64>().unwrap();
    let ts = convert_micros(timestamp);
    return ts.to_rfc3339();
}


#[wasm_bindgen]
pub fn now_rfc3339() -> String {
   let datetime: DateTime<Local> = Local::now();
   return datetime.to_rfc3339();  
}

#[wasm_bindgen]
pub fn now() -> String {
   let datetime: DateTime<Local> = Local::now();
   let format = "%s%6f";
   return datetime.format(format).to_string();  
}


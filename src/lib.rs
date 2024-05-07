mod utils;
use wasm_bindgen::prelude::*;
use wat::parse_str;

#[wasm_bindgen]
pub fn parse_wat(str: &str) -> Result<Vec<u8>, String> {
    match parse_str(str) {
        Ok(bytes) => Ok(bytes),
        Err(error) => Err(error.to_string()),
    }
}

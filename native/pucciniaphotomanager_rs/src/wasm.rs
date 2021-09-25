use crate::logic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_exif(bytes: &[u8]) -> String {
    logic::parse_exif(bytes)
}

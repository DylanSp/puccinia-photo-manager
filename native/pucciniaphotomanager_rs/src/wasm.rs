use crate::logic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmExifParseResult {
    #[wasm_bindgen(getter_with_clone)]
    pub artist: Option<String>,

    #[wasm_bindgen(getter_with_clone)]
    pub image_description: Option<String>,
}

#[wasm_bindgen]
pub fn parse_exif(bytes: &[u8]) -> Option<WasmExifParseResult> {
    logic::parse_exif(bytes).map(|parse_result| WasmExifParseResult {
        artist: parse_result.artist,
        image_description: parse_result.image_description,
    })
}

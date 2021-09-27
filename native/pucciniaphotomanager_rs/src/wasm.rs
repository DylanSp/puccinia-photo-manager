use crate::logic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct WasmImageResolution {
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
pub struct WasmExifParseResult {
    pub resolution: Option<WasmImageResolution>,
    pub date_created_unix_seconds: Option<i64>,
}

#[wasm_bindgen]
pub fn parse_exif(bytes: &[u8]) -> Option<WasmExifParseResult> {
    logic::parse_exif(bytes).map(|parse_result| WasmExifParseResult {
        resolution: parse_result
            .resolution
            .map(|parse_result_resolution| WasmImageResolution {
                width: parse_result_resolution.width,
                height: parse_result_resolution.height,
            }),
        date_created_unix_seconds: parse_result.date_created_unix_seconds,
    })
}

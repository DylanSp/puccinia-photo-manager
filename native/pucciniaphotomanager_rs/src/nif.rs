use rustler::{Binary, NifStruct};

use crate::logic;

#[derive(NifStruct)]
#[module = "Elixir.PucciniaPhotoManager.Nif.NifExifParseResult"]
pub struct NifExifParseResult {
    resolution: Option<NifImageResolution>,
    date_created_unix_seconds: Option<i64>,
}

#[derive(NifStruct)]
#[module = "Elixir.PucciniaPhotoManager.Nif.NifImageResolution"]
struct NifImageResolution {
    pub width: u32,
    pub height: u32,
}

#[rustler::nif]
pub fn parse_exif(bytes: Binary) -> Option<NifExifParseResult> {
    logic::parse_exif(&bytes).map(|parse_result| NifExifParseResult {
        resolution: parse_result
            .resolution
            .map(|parse_result_resolution| NifImageResolution {
                width: parse_result_resolution.width,
                height: parse_result_resolution.height,
            }),
        date_created_unix_seconds: parse_result.date_created_unix_seconds,
    })
}

rustler::init!("Elixir.PucciniaPhotoManager.Nif", [parse_exif]);

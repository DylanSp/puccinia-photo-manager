use rustler::{Binary, NifStruct};

use crate::logic;

#[derive(NifStruct)]
#[module = "Elixir.PucciniaPhotoManager.Nif.NifExifParseResult"]
pub struct NifExifParseResult {
    artist: Option<String>,
    image_description: Option<String>,
}

#[rustler::nif]
pub fn parse_exif(bytes: Binary) -> Option<NifExifParseResult> {
    logic::parse_exif(&bytes).map(|parse_result| NifExifParseResult {
        artist: parse_result.artist,
        image_description: parse_result.image_description,
    })
}

rustler::init!("Elixir.PucciniaPhotoManager.Nif", [parse_exif]);

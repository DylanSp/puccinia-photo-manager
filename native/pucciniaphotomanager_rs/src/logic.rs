use exif::{In, Tag};

pub struct ExifParseResult {
    pub artist: Option<String>,
    pub image_description: Option<String>,
    // date_created: Option<DateTime>
}

pub fn parse_exif(bytes: &[u8]) -> Option<ExifParseResult> {
    let exifreader = exif::Reader::new();
    let mut cursor = std::io::Cursor::new(bytes);
    match exifreader.read_from_container(&mut cursor) {
        Ok(exif) => {
            let artist = exif
                .get_field(Tag::Artist, In::PRIMARY)
                .map(|field| field.display_value().to_string());

            let image_description = exif
                .get_field(Tag::ImageDescription, In::PRIMARY)
                .map(|field| field.display_value().to_string());

            Some(ExifParseResult {
                artist,
                image_description,
            })
        }
        Err(_) => None,
    }
}

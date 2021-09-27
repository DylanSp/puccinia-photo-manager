use chrono::{DateTime, Utc};
use exif::{Context, Exif, In, Tag};

pub struct ImageResolution {
    pub width: u32,
    pub height: u32,
}

pub struct ExifParseResult {
    pub resolution: Option<ImageResolution>,
    pub date_created_unix_seconds: Option<i64>,
}

fn get_exif_resolution(exif: &Exif) -> Option<ImageResolution> {
    let exif_image_width_tag = Tag(Context::Exif, 40962);
    let exif_image_height_tag = Tag(Context::Exif, 40963);

    let width = exif
        .get_field(exif_image_width_tag, In::PRIMARY)?
        .value
        .get_uint(0)?;
    let height = exif
        .get_field(exif_image_height_tag, In::PRIMARY)?
        .value
        .get_uint(0)?;
    Some(ImageResolution { width, height })
}

pub fn parse_exif(bytes: &[u8]) -> Option<ExifParseResult> {
    let exifreader = exif::Reader::new();
    let mut cursor = std::io::Cursor::new(bytes);
    let exif = exifreader.read_from_container(&mut cursor).ok()?;

    let resolution = get_exif_resolution(&exif);

    let date_str = exif
        .get_field(Tag::GPSDateStamp, In::PRIMARY)?
        .display_value()
        .to_string();
    let time_str = exif
        .get_field(Tag::GPSTimeStamp, In::PRIMARY)?
        .display_value()
        .to_string();
    let unix_timestamp = format!("{}T{}Z", date_str, time_str)
        .parse::<DateTime<Utc>>()
        .ok()
        .map(|datetime| datetime.timestamp());

    Some(ExifParseResult {
        resolution,
        date_created_unix_seconds: unix_timestamp,
    })
}

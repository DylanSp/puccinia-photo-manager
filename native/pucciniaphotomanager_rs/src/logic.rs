pub fn parse_exif(bytes: &[u8]) -> String {
    let exifreader = exif::Reader::new();
    let mut cursor = std::io::Cursor::new(bytes);
    match exifreader.read_from_container(&mut cursor) {
        Ok(exif) => {
            let x = exif
                .fields()
                .map(|field| {
                    format!(
                        "{} {} {}",
                        field.tag,
                        field.ifd_num,
                        field.display_value().with_unit(&exif)
                    )
                })
                .fold("".to_owned(), |acc, str| format!("{}\n{}", acc, str));
            return x;
        }
        Err(err) => format!("{}", err),
    }
}

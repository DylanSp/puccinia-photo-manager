use crate::logic;

#[rustler::nif]
pub fn parse_exif(/* bytes: &[u8]*/ str: String) -> String {
    str
    // logic::parse_exif(bytes)
}

rustler::init!("Elixir.PucciniaPhotoManager.Nif", [parse_exif]);

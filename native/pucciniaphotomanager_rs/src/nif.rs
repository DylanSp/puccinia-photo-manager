use crate::logic

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
  logic::add(a, b)
}

rustler::init!("Elixir.PucciniaPhotoManager.Nif", [add]);

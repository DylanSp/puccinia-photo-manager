defmodule PucciniaPhotoManager.Nif do
  use Rustler, otp_app: :puccinia_photo_manager, crate: "pucciniaphotomanager_rs"

  def parse_exif(_str) do
    :erlang.nif_error(":nif_not_loaded")
  end
end

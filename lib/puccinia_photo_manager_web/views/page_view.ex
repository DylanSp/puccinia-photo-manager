defmodule PucciniaPhotoManagerWeb.PageView do
  use PucciniaPhotoManagerWeb, :view

  import PucciniaPhotoManager.Nif

  def get_greeting_message() do
    bytes = File.read!("/hdd/programming/elixir/puccinia_photo_manager/assets/images/exif.jpg")
    parse_result = parse_exif(bytes)
    artist = parse_result.artist || "Unknown artist"
    description = parse_result.image_description || "Unknown image description"
    "Artist: #{artist}, description: #{description}"
  end
end

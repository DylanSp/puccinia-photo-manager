defmodule PucciniaPhotoManagerWeb.PageView do
  use PucciniaPhotoManagerWeb, :view

  import PucciniaPhotoManager.Nif

  def get_greeting_message() do
    parse_exif("Hello!")
  end
end

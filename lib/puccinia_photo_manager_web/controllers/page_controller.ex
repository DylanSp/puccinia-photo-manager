defmodule PucciniaPhotoManagerWeb.PageController do
  use PucciniaPhotoManagerWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end

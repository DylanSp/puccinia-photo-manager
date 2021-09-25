defmodule PucciniaPhotoManager.Repo do
  use Ecto.Repo,
    otp_app: :puccinia_photo_manager,
    adapter: Ecto.Adapters.Postgres
end

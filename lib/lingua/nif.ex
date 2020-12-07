defmodule Lingua.Nif do
  @moduledoc false

  use Rustler, otp_app: :lingua, crate: "lingua_nif"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end

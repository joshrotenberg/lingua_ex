defmodule Lingua.Nif do
  @moduledoc false

  use Rustler, otp_app: :lingua, crate: "lingua_nif"

  def detect_language_of(_text, _builder_option, _languages, _minimum_relative_distance), do: :erlang.nif_error(:nif_not_loaded)
end

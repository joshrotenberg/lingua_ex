defmodule Lingua.Nif do
  @moduledoc false

  use Rustler, otp_app: :lingua, crate: "lingua_nif"

  def init(), do: error()

  def detect_language_of(_text, _builder_option, _languages, _minimum_relative_distance),
    do: error()

  def compute_language_confidence_values(
        _text,
        _builder_option,
        _languages,
        _minimum_relative_distance
      ),
      do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
end

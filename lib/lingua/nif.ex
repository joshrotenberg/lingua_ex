defmodule Lingua.Nif do
  @moduledoc false

  use Rustler, otp_app: :lingua, crate: "lingua_nif"

  def init(), do: error()

  def run_detection(
        _text,
        _builder_option,
        _languages,
        _compute_language_confidence_values,
        _minimum_relative_distance,
        _preload_language_models,
        _low_accuracy_mode
      ),
      do: error()

  def all_languages(), do: error()
  def all_spoken_languages(), do: error()
  def all_languages_with_arabic_script(), do: error()
  def all_languages_with_cyrillic_script(), do: error()
  def all_languages_with_devanagari_script(), do: error()
  def all_languages_with_latin_script(), do: error()
  def language_for_iso_code(_code), do: error()
  def language_for_iso_code_639_1(_code), do: error()
  def language_for_iso_code_639_3(_code), do: error()
  def iso_code_639_1_for_language(_language), do: error()
  def iso_code_639_3_for_language(_language), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
end

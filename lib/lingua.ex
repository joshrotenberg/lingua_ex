defmodule Lingua do
  @moduledoc """
  Lingua wraps [Peter M. Stahl](https://github.com/pemistahl)'s [linuga-rs](https://github.com/pemistahl/lingua-rs) language detection library.
  This wrapper follows the lingua-rs API closely, so consult the [documentation](https://docs.rs/lingua/1.0.3/lingua/index.html) for more information.
  """

  @default_builder_option :all_languages
  @default_languages []
  @default_minimum_relative_distance 0.0
  @default_compute_language_confidence_values false
  @default_preload_language_models false
  @default_low_accuracy_mode false

  @doc """
  Initialize the detector. Calling this is optional but it may come in handy in cases where you want lingua-rs to load
  the language corpora so that subsequent calls to `detect` are fast. The first time the detector is run it can take some time to load (~12 seconds on my Macbook Pro). See
  also the `preload_language_models` option below.

  ## Example

      iex> Lingua.init()
      :ok
  """
  defdelegate init(), to: Lingua.Nif

  @doc """
  Detect the language of the given input text. By default, all supported languages will be considered and
  the minimum relative distance is `0.0`.

  Returns the detected language, or a list of languages and their confidence values, or `:no_match` if the given text
  doesn't match a language.

  Options:

  * `builder_option:` - can be one of the following (defaults to `:all_languages`):
    * `:all_languages` - consider every supported language
    * `:all_spoken_languages` - consider only currently spoken languages
    * `:all_languages_with_arabic_script` - consider only languages written in Arabic script
    * `:all_languages_with_cyrillic_script` - consider only languages written in Cyrillic script
    * `:all_languages_with_devanagari_script` - consider only languages written in Devanagari script
    * `:all_languages_with_latin_script` - consider only languages written in Latin script
    * `:with_languages` - consider only the languages supplied in the `languages` option.
    * `:without_languages` - consider all languages except those supplied in the `languages` option. Two or more are required. (see below)

  * `languages:` - specify two or more languages to consider or to not consider depending on the `builder_option:` (defaults to `[]`). Accepts
  any combination of languages and ISO 639-1 or 639-3 codes, for example: `[:english, :ru, :lit]` to consider English, Russian and Lithuanian.

  * `minimum_relative_distance:` - specify the minimum relative distance (0.0 - 0.99) required for a language to be considered a match for the input.
  See the lingua-rs [documentation](https://docs.rs/lingua/1.0.3/lingua/struct.LanguageDetectorBuilder.html#method.with_minimum_relative_distance) for details. (defaults to `0.0`)

  * `compute_language_confidence_values:` - returns the full list of language matches for the input and their confidence values. (defaults to `false`)

  * `preload_language_models:` - preload all language models instead of just those required for the match. (defaults to `false`)

  * `low_accuracy_mode:` - use low accuracy mode for faster detection at the cost of accuracy. (defaults to `false`)

  ## Examples

      iex> Lingua.detect("this is definitely English")
      {:ok, :english}

      iex> Lingua.detect("וזה בעברית")
      {:ok, :hebrew}

      iex> Lingua.detect("państwowych", builder_option: :with_languages, languages: [:english, :russian, :polish])
      {:ok, :polish}

      iex> Lingua.detect("ѕидови", builder_option: :all_languages_with_cyrillic_script)
      {:ok, :macedonian}

      iex> Lingua.detect("כלב", builder_option: :with_languages, languages: [:english, :russian, :polish])
      {:ok, :no_match}

      iex> Lingua.detect("what in the world is this", builder_option: :with_languages, languages: [:english, :russian, :hebrew], compute_language_confidence_values: true)
      {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

  """
  def detect(text, options \\ []) do
    builder_option = Keyword.get(options, :builder_option, @default_builder_option)
    languages = Keyword.get(options, :languages, @default_languages)

    minimum_relative_distance =
      Keyword.get(options, :minimum_relative_distance, @default_minimum_relative_distance)

    preload_language_models =
      Keyword.get(options, :preload_language_models, @default_preload_language_models)

    compute_language_confidence_values =
      Keyword.get(
        options,
        :compute_language_confidence_values,
        @default_compute_language_confidence_values
      )

    low_accuracy_mode = Keyword.get(options, :low_accuracy_mode, @default_low_accuracy_mode)

    Lingua.Nif.run_detection(
      text,
      builder_option,
      languages,
      compute_language_confidence_values,
      minimum_relative_distance,
      preload_language_models,
      low_accuracy_mode
    )
  end

  @doc """
  Like `detect`, but returns the result value or raises an error.
  """
  def detect!(text, options \\ []) do
    case detect(text, options) do
      {:ok, value} -> value
      {:error, error} -> raise error
    end
  end

  @doc """
  Get the list of supported languages.

  ## Example

      iex> Lingua.all_languages()
      [:afrikaans, :albanian, :arabic, :armenian, :azerbaijani, :basque, :belarusian,
       :bengali, :bokmal, :bosnian, :bulgarian, :catalan, :chinese, :croatian, :czech,
       :danish, :dutch, :english, :esperanto, :estonian, :finnish, :french, :ganda,
       :georgian, :german, :greek, :gujarati, :hebrew, :hindi, :hungarian, :icelandic,
       :indonesian, :irish, :italian, :japanese, :kazakh, :korean, :latin, :latvian,
       :lithuanian, :macedonian, :malay, :maori, :marathi, :mongolian, :nynorsk,
       :persian, :polish, :portuguese, :punjabi, :romanian, :russian, :serbian,
       :shona, :slovak, :slovene, :somali, :sotho, :spanish, :swahili, :swedish,
       :tagalog, :tamil, :telugu, :thai, :tsonga, :tswana, :turkish, :ukrainian,
       :urdu, :vietnamese, :welsh, :xhosa, :yoruba, :zulu]
  """
  defdelegate all_languages(), to: Lingua.Nif

  @doc """
  Get the list of supported spoken languages.

  ## Example

      iex> Lingua.all_spoken_languages()
      [:afrikaans, :albanian, :arabic, :armenian, :azerbaijani, :basque, :belarusian,
       :bengali, :bokmal, :bosnian, :bulgarian, :catalan, :chinese, :croatian, :czech,
       :danish, :dutch, :english, :esperanto, :estonian, :finnish, :french, :ganda,
       :georgian, :german, :greek, :gujarati, :hebrew, :hindi, :hungarian, :icelandic,
       :indonesian, :irish, :italian, :japanese, :kazakh, :korean, :latvian,
       :lithuanian, :macedonian, :malay, :maori, :marathi, :mongolian, :nynorsk,
       :persian, :polish, :portuguese, :punjabi, :romanian, :russian, :serbian,
       :shona, :slovak, :slovene, :somali, :sotho, :spanish, :swahili, :swedish,
       :tagalog, :tamil, :telugu, :thai, :tsonga, :tswana, :turkish, :ukrainian,
       :urdu, :vietnamese, :welsh, :xhosa, :yoruba, :zulu]
  """
  defdelegate all_spoken_languages(), to: Lingua.Nif

  @doc """
  Get the list of supported languages using Arabic script.

  ## Example

      iex> Lingua.all_languages_with_arabic_script()
      [:arabic, :persian, :urdu]
  """
  defdelegate all_languages_with_arabic_script(), to: Lingua.Nif

  @doc """
  Get the list of supported languages using Cyrillic script.

  ## Example

      iex> Lingua.all_languages_with_cyrillic_script()
      [:belarusian, :bulgarian, :kazakh, :macedonian, :mongolian, :russian, :serbian, :ukrainian]
  """
  defdelegate all_languages_with_cyrillic_script(), to: Lingua.Nif

  @doc """
  Get the list of supported languages using Devanagari script.

  ## Example

      iex> Lingua.all_languages_with_devanagari_script()
      [:hindi, :marathi]
  """
  defdelegate all_languages_with_devanagari_script(), to: Lingua.Nif

  @doc """
  Get the list of supported languages using Latin script.

  ## Example

      iex> Lingua.all_languages_with_latin_script()
      [:afrikaans, :albanian, :azerbaijani, :basque, :bokmal, :bosnian, :catalan,
       :croatian, :czech, :danish, :dutch, :english, :esperanto, :estonian, :finnish,
       :french, :ganda, :german, :hungarian, :icelandic, :indonesian, :irish,
       :italian, :latin, :latvian, :lithuanian, :malay, :maori, :nynorsk, :polish,
       :portuguese, :romanian, :shona, :slovak, :slovene, :somali, :sotho, :spanish,
       :swahili, :swedish, :tagalog, :tsonga, :tswana, :turkish, :vietnamese, :welsh,
       :xhosa, :yoruba, :zulu]
  """
  defdelegate all_languages_with_latin_script(), to: Lingua.Nif

  @doc """
  Get the language for the given ISO 639-1 language code.

  ## Example

      iex> Lingua.language_for_iso_code_639_1(:en)
      {:ok, :english}
      iex> Lingua.language_for_iso_code_639_1(:er)
      {:error, :unrecognized_iso_code}
  """
  defdelegate language_for_iso_code_639_1(code), to: Lingua.Nif

  @doc """
  Get the language for the given ISO 639-3 language code.

  ## Example

      iex> Lingua.language_for_iso_code_639_3(:eng)
      {:ok, :english}
      iex> Lingua.language_for_iso_code_639_3(:enr)
      {:error, :unrecognized_iso_code}
  """
  defdelegate language_for_iso_code_639_3(code), to: Lingua.Nif

  @doc """
  Get the language for the given ISO 639-1 or 639-3 language code.

  ## Example

      iex> Lingua.language_for_iso_code(:en)
      {:ok, :english}
      iex> Lingua.language_for_iso_code(:eng)
      {:ok, :english}
      iex> Lingua.language_for_iso_code(:mop)
      {:error, :unrecognized_iso_code}
  """
  defdelegate language_for_iso_code(code), to: Lingua.Nif

  @doc """
  Get the ISO 639-1 language code for the given language.

  ## Example

      iex> Lingua.iso_code_639_1_for_language(:english)
      {:ok, :en}
      iex> Lingua.iso_code_639_1_for_language(:nope)
      {:error, :unrecognized_language}
  """
  defdelegate iso_code_639_1_for_language(language), to: Lingua.Nif

  @doc """
  Get the ISO 639-3 language code for the given language.

  ## Example

      iex> Lingua.iso_code_639_3_for_language(:english)
      {:ok, :eng}
      iex> Lingua.iso_code_639_1_for_language(:nope)
      {:error, :unrecognized_language}
  """
  defdelegate iso_code_639_3_for_language(language), to: Lingua.Nif
end

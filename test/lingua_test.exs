defmodule LinguaTest do
  use ExUnit.Case, async: true
  @moduletag timeout: :infinity
  doctest Lingua

  @total_language_count 75
  @spoken_language_count 74
  @arabic_count 3
  @cyrillic_count 8
  @devanagari_count 2
  @latin_count 49

  setup_all do
    Lingua.init()
    :ok
  end

  describe "utility tests" do
    test "all_languages/0" do
      assert Lingua.all_languages() |> Enum.member?(:english)
      assert Lingua.all_languages() |> Enum.member?(:russian)
      assert Lingua.all_languages() |> Enum.member?(:hebrew)

      assert Lingua.all_languages()
             |> Enum.count() == @total_language_count
    end

    test "all_spoken_languages/0" do
      assert Lingua.all_spoken_languages() |> Enum.member?(:english)
      assert Lingua.all_spoken_languages() |> Enum.member?(:russian)
      assert Lingua.all_spoken_languages() |> Enum.member?(:hebrew)

      assert Lingua.all_spoken_languages()
             |> Enum.count() == @spoken_language_count
    end

    test "all_languages_with_arabic_script/0" do
      assert Lingua.all_languages_with_arabic_script() |> Enum.member?(:arabic)
      assert Lingua.all_languages_with_arabic_script() |> Enum.member?(:persian)
      assert Lingua.all_languages_with_arabic_script() |> Enum.member?(:urdu)

      assert Lingua.all_languages_with_arabic_script()
             |> Enum.count() == @arabic_count
    end

    test "all_languages_with_cyrillic_script/0" do
      assert Lingua.all_languages_with_cyrillic_script() |> Enum.member?(:russian)
      assert Lingua.all_languages_with_cyrillic_script() |> Enum.member?(:ukrainian)
      assert Lingua.all_languages_with_cyrillic_script() |> Enum.member?(:macedonian)

      assert Lingua.all_languages_with_cyrillic_script()
             |> Enum.count() == @cyrillic_count
    end

    test "all_languages_with_devanagari_script/0" do
      assert Lingua.all_languages_with_devanagari_script() |> Enum.member?(:hindi)
      assert Lingua.all_languages_with_devanagari_script() |> Enum.member?(:marathi)

      assert Lingua.all_languages_with_devanagari_script()
             |> Enum.count() == @devanagari_count
    end

    test "all_languages_with_latin_script/0" do
      assert Lingua.all_languages_with_latin_script() |> Enum.member?(:welsh)
      assert Lingua.all_languages_with_latin_script() |> Enum.member?(:bosnian)
      assert Lingua.all_languages_with_latin_script() |> Enum.member?(:swedish)

      assert Lingua.all_languages_with_latin_script()
             |> Enum.count() == @latin_count
    end

    test "language_for_iso_code_639_1/1" do
      assert Lingua.language_for_iso_code_639_1(:en) == {:ok, :english}
      assert Lingua.language_for_iso_code_639_1(:er) == {:error, :unrecognized_iso_code}
    end

    test "language_for_iso_code_639_3/1" do
      assert Lingua.language_for_iso_code_639_3(:eng) == {:ok, :english}
      assert Lingua.language_for_iso_code_639_3(:nope) == {:error, :unrecognized_iso_code}
    end

    test "language_for_iso_code/1" do
      assert Lingua.language_for_iso_code(:en) == {:ok, :english}
      assert Lingua.language_for_iso_code(:er) == {:error, :unrecognized_iso_code}
      assert Lingua.language_for_iso_code(:eng) == {:ok, :english}
      assert Lingua.language_for_iso_code(:nope) == {:error, :unrecognized_iso_code}
    end

    test "iso_code_639_1_for_language/1" do
      assert Lingua.iso_code_639_1_for_language(:english) == {:ok, :en}
      assert Lingua.iso_code_639_1_for_language(:nope) == {:error, :unrecognized_language}
    end

    test "iso_code_639_3_for_language/1" do
      assert Lingua.iso_code_639_3_for_language(:english) == {:ok, :eng}
      assert Lingua.iso_code_639_3_for_language(:nope) == {:error, :unrecognized_language}
    end
  end

  describe "detection tests" do
    test "init/0" do
      assert Lingua.init() == :ok
    end

    test "detect/1" do
      assert Lingua.detect("this is definitely English") == {:ok, :english}
      assert Lingua.detect("וזה בעברית") == {:ok, :hebrew}
    end

    test "detect/1 with empty string" do
      assert Lingua.detect("") == {:ok, :no_match}
    end

    test "detect/1 with very short text" do
      # Single characters may still be detected (lingua tries its best)
      {:ok, result} = Lingua.detect("a")
      assert is_atom(result)
    end

    test "detect!/1" do
      assert Lingua.detect!("this is definitely English") == :english
      assert Lingua.detect!("וזה בעברית") == :hebrew
    end

    test "detect!/2" do
      assert Lingua.detect!("this is English",
               builder_option: :with_languages,
               languages: [:english, :german]
             ) == :english
    end

    test "detect!/2 raises on error" do
      assert_raise ArgumentError, ~r/insufficient_languages/, fn ->
        Lingua.detect!("hello", builder_option: :with_languages, languages: [:english])
      end
    end

    test "detect/2 with without_languages option" do
      # Exclude English, should still detect something for English text
      result =
        Lingua.detect("this is definitely some text",
          builder_option: :without_languages,
          languages: [:german, :french]
        )

      assert {:ok, lang} = result
      assert lang != :german
      assert lang != :french
    end

    test "detect/2" do
      assert Lingua.detect("państwowych",
               builder_option: :with_languages,
               languages: [:english, :russian, :polish]
             ) == {:ok, :polish}

      assert Lingua.detect("państwowych",
               builder_option: :with_languages,
               languages: [:eng, :russian, :pol]
             ) == {:ok, :polish}

      assert Lingua.detect("ѕидови", builder_option: :all_languages_with_cyrillic_script) ==
               {:ok, :macedonian}

      assert Lingua.detect("כלב",
               builder_option: :with_languages,
               languages: [:english, :russian, :polish]
             ) == {:ok, :no_match}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:english, :russian, :hebrew],
               compute_language_confidence_values: true
             ) ==
               {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:en, :ru, :he],
               compute_language_confidence_values: true
             ) ==
               {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng, :rus, :heb],
               compute_language_confidence_values: true
             ) ==
               {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng, :rus, :heb],
               compute_language_confidence_values: true,
               preload_language_models: true
             ) ==
               {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng, :rus, :heb],
               compute_language_confidence_values: true,
               preload_language_models: false
             ) ==
               {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng, :rus, :heb],
               compute_language_confidence_values: true,
               preload_language_models: false,
               low_accuracy_mode: true
             ) ==
               {:ok, [{:english, 1.0}, {:hebrew, 0.0}, {:russian, 0.0}]}

      assert_raise ArgumentError, fn ->
        Lingua.detect("what in the world is this",
          builder_option: :with_languages,
          languages: [:kqjwenbg, :rus, :heb],
          compute_language_confidence_values: true
        )
      end

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng]
             ) ==
               {:error, :insufficient_languages}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng, :rus],
               minimum_relative_distance: 1.1
             ) ==
               {:error, :out_of_range_minimum_relative_distance}

      assert Lingua.detect("what in the world is this",
               builder_option: :with_languages,
               languages: [:eng, :rus],
               minimum_relative_distance: -0.1
             ) ==
               {:error, :out_of_range_minimum_relative_distance}
    end
  end
end

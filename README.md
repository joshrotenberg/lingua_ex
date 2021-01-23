# Lingua

[![Hex pm](http://img.shields.io/hexpm/v/lingua.svg?style=flat)](https://hex.pm/packages/lingua)
![Elixir CI](https://github.com/joshrotenberg/lingua_ex/workflows/Elixir%20CI/badge.svg)
![License](https://img.shields.io/hexpm/l/lingua)
![Hex.pm](https://img.shields.io/hexpm/dw/lingua)
[![Coverage Status](https://coveralls.io/repos/github/joshrotenberg/lingua_ex/badge.svg?branch=master)](https://coveralls.io/github/joshrotenberg/lingua_ex?branch=master)

Lingua is an [Elixir][0] wrapper for the [Rust][1] [lingua][2] crate with [Rustler][3].

## Summary

Lingua is a NIF-based bridge for the [lingua][2] [Rust][1] language detection library.

## Usage

```
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
{:ok, [english: 1.0]}
```

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `lingua_ex` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:lingua, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/lingua_ex](https://hexdocs.pm/lingua_ex).


[0]: https://elixir-lang.org
[1]: https://www.rust-lang.org 
[2]: https://crates.io/crates/lingua
[3]: https://hex.pm/packages/rustler
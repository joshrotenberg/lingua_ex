defmodule Lingua.MixProject do
  use Mix.Project

  def project do
    [
      app: :lingua,
      version: "0.1.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      description: description(),
      package: package(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      deps: deps()
    ]
  end

  defp description do
    "An Elixir wrapper around the Rust Lingua language detection library."
  end

  defp package do
    [
      maintainers: ["Josh Rotenberg"],
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => "https://github.com/joshrotenberg/lingua_ex"}
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp rustler_crates do
    [
      lingua_nif: [
        mode: rustc_mode(Mix.env())
      ]
    ]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(:bench), do: :release
  defp rustc_mode(_), do: :debug

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.1"},
      {:credo, "~> 1.4", only: [:dev, :test], runtime: false},
    ]
  end
end

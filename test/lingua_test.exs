defmodule LinguaTest do
  use ExUnit.Case
  doctest Lingua

  test "greets the world" do
    assert Lingua.hello() == :world
  end
end

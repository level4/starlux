defmodule StarluxTest do
  use ExUnit.Case
  doctest Starlux

  test "greets the world" do
    assert Starlux.hello() == :world
  end
end

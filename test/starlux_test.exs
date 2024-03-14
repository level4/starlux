defmodule StarluxTest do
  use ExUnit.Case
  doctest Starlux

  test "greets the world" do
    assert Starlux.hello() == :world
  end

  test "evaluates and returns json" do
    code = "emit(1 + 1)"
    assert Starlux.Run.evaluate_and_return_json(code) == {:ok, ["2"]}
  end

  test "evaluates more complex code" do
    code = """
    def add(x, y):
      return x + y
    emit(add(1, 2))
    """

    assert Starlux.Run.evaluate_and_return_json(code) == {:ok, ["3"]}
  end

  test "evaluates emit of a map" do
    code = """
    emit({"a": 1, "b": 2})
    """

    assert Starlux.Run.evaluate_and_return_json(code) == {:ok, ["{\"a\":1,\"b\":2}"]}
  end
end

defmodule StarluxTest do
  use ExUnit.Case
  doctest Starlux

  test "greets the world" do
    assert Starlux.hello() == :world
  end

  test "evaluates and returns json" do
    code = "emit(1 + 1)"
    assert Starlux.Run.evaluate(code) == {:ok, {"None", ["2"]}}
  end

  test "evaluates more complex code" do
    code = """
    def add(x, y):
      return x + y
    res = add(1, 2)
    length = len(str(res))
    emit(res)
    emit(length)
    res
    """

    assert Starlux.Run.evaluate(code) == {:ok, {"3", ["3", "1"]}}
  end

  test "evaluates emit of a map" do
    code = """
    emit({"a": 1, "b": 2})
    """

    assert Starlux.Run.evaluate(code) == {:ok, {"None", ["{\"a\":1,\"b\":2}"]}}
  end
end

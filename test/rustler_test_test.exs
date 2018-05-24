defmodule RustlerTestTest do
  use ExUnit.Case
  doctest RustlerTest

  test "greets the world" do
    assert RustlerTest.hello() == :world
  end
end

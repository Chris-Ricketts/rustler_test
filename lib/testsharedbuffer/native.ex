defmodule RustlerTest.Native do
  use Rustler, otp_app: :rustler_test, crate: :testsharedbuffer

  def add(_x, _y), do: error()
	def create(_size), do: error()
	def get(_buffer, _idx), do: error()
	def set(_buffer, _idx, _value), do: error()
	def print(_buffer), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end

defmodule Starlux.Run do
  use Rustler, otp_app: :starlux, crate: "starlux_run"

  # When your NIF is loaded, it will override this function.
  def evaluate(_code), do: :erlang.nif_error(:nif_not_loaded)
end

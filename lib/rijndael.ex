defmodule Rijndael do
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :rijndael,
    crate: "rijndael",
    base_url: "https://***REMOVED***github.com/ringvold/rijndael/releases/download/v#{version}",
    version: version


  # When your NIF is loaded, it will override this function.
  def decrypt(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end

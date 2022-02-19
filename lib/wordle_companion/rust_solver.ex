defmodule WordleCompanion.RustSolver do
  use Rustler,
    otp_app: :wordle_companion,
    crate: "rustsolver",
    load_data:
      Application.get_env(
        :wordle_companion,
        WordleCompanion.RustSolver,
        :word_file_path
      )

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def external_calc(_solution, _guesses), do: :erlang.nif_error(:nif_not_loaded)
end

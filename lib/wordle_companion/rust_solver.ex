defmodule WordleCompanion.RustSolver do
  use Rustler,
    otp_app: :wordle_companion,
    crate: "rustsolver"

  def external_calc(_solution, _guesses), do: :erlang.nif_error(:nif_not_loaded)
  def external_words(_solution, _guesses), do: :erlang.nif_error(:nif_not_loaded)
end

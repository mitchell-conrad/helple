defmodule WordleCompanionWeb.Counter do
  use WordleCompanionWeb, :live_view
  require Logger

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       solution: "",
       guesses: ["", "", "", "", "", ""],
       remaining: [0, 0, 0, 0, 0, 0],
       calc_time: 0,
       remaining_words: []
     )}
  end

  def handle_event("eval-guesses", session, socket) do
    guesses = Enum.map(0..5, fn x -> session[Integer.to_string(x)] end)
    solution = session["solution_field"]

    {time, nums} =
      :timer.tc(&WordleCompanion.RustSolver.external_calc/2, [
        solution,
        guesses
      ])

    Logger.info(%{operation: "eval-guesses", time: time / 1000, vals: nums, guesses: guesses})

    {:noreply,
     assign(socket, %{
       solution: solution,
       guesses: guesses,
       remaining: nums,
       calc_time: time
     })}
  end

  def handle_event("get-words", session, socket) do
    guesses = Enum.map(0..5, fn x -> session[Integer.to_string(x)] end)
    solution = session["solution_field"]

    {time, words} =
      :timer.tc(&WordleCompanion.RustSolver.external_words/2, [
        solution,
        guesses
      ])

    words = Enum.join(words, " ")

    Logger.info(%{operation: "get-words", time: time / 1000, vals: words})

    {:noreply,
     assign(socket, %{
       solution: solution,
       guesses: guesses,
       calc_time: time,
       remaining_words: words
     })}
  end

  def render(assigns) do
    render(WordleCompanionWeb.PageView, "index.html", assigns)
  end
end

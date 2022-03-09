defmodule WordleCompanionWeb.RemainingWords do
  use WordleCompanionWeb, :live_view
  require Logger

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       solution: "",
       guesses: ["", "", "", "", "", ""],
       remaining: [0, 0, 0, 0, 0, 0],
       calc_time: 0,
       remaining_words: [],
       guess_histogram: ["", "", "", "", "", "", ""],
       mean: 0,
       std_dev: 0,
       state: :remaining
     )}
  end

  def handle_event("mode-remaining", session, socket) do
    {:noreply, assign(socket, state: :remaining)}
  end

  def handle_event("mode-stats", session, socket) do
    {:noreply, assign(socket, state: :stats)}
  end

  def handle_event("eval-guesses", session, socket) do
    guesses = Enum.map(0..5, fn x -> session[Integer.to_string(x)] end)
    solution = session["solution_field"]

    {time, nums} =
      :timer.tc(&WordleCompanion.RustSolver.external_calc/2, [
        solution,
        guesses
      ])

    Logger.info(
      operation: "eval-guesses",
      time: time / 1000,
      vals: nums,
      guesses: guesses
    )

    {:noreply,
     assign(socket,
       solution: solution,
       guesses: guesses,
       remaining: nums,
       calc_time: time
     )}
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

    Logger.info(
      operation: "get-words",
      time: time / 1000,
      vals: words
    )

    {:noreply,
     assign(socket, %{
       solution: solution,
       guesses: guesses,
       calc_time: time,
       remaining_words: words
     })}
  end

  def handle_event("eval-stats", session, socket) do
    guess_fields =
      Enum.map(
        0..5,
        fn x -> session["h" <> Integer.to_string(x)] end
      )

    guess_histogram =
      Enum.map(guess_fields, fn f ->
        case Integer.parse(f) do
          {v, _} -> v
          :error -> 0
        end
      end)

    guess_histogram = [0 | guess_histogram]

    mean =
      WordleCompanion.RustSolver.external_mean(guess_histogram)
      |> Float.round(3)

    std_dev =
      WordleCompanion.RustSolver.external_std_dev(guess_histogram)
      |> Float.round(3)

    Logger.info(mean: mean, std_dev: std_dev)

    {:noreply,
     assign(socket,
       mean: mean,
       std_dev: std_dev,
       guess_histogram: guess_histogram
     )}
  end

  def render(assigns) do
    case assigns[:state] do
      :remaining -> render(WordleCompanionWeb.PageView, "remaining.html", assigns)
      :stats -> render(WordleCompanionWeb.PageView, "stats.html", assigns)
    end
  end
end

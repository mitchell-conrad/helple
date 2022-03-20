defmodule HelpleWeb.RemainingWords do
  use HelpleWeb, :live_view

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       solution: "",
       guesses: ["", "", "", "", "", ""],
       remaining: [0, 0, 0, 0, 0, 0],
       remaining_words: [],
       guess_histogram: ["", "", "", "", "", "", ""],
       mean: 0,
       std_dev: 0,
       count: 0,
       state: :remaining
     )}
  end

  def handle_event("mode-remaining", _session, socket) do
    {:noreply, assign(socket, state: :remaining)}
  end

  def handle_event("mode-stats", _session, socket) do
    {:noreply, assign(socket, state: :stats)}
  end

  def handle_event("eval-guesses", session, socket) do
    {solution, guesses} = parse_remaining_words_input(session)

    {:noreply,
     assign(socket,
       solution: solution,
       guesses: guesses,
       remaining: Helple.RustSolver.external_calc(solution, guesses)
     )}
  end

  def handle_event("get-words", session, socket) do
    {solution, guesses} = parse_remaining_words_input(session)

    {:noreply,
     assign(socket,
       solution: solution,
       guesses: guesses,
       remaining_words:
         Enum.join(
           Helple.RustSolver.external_words(solution, guesses),
           " "
         )
     )}
  end

  def handle_event("eval-stats", session, socket) do
    guess_histogram = build_histogram(session)

    {:noreply,
     assign(socket,
       guess_histogram: guess_histogram,
       std_dev: calc_std_dev(guess_histogram),
       count: calc_n(guess_histogram),
       mean: calc_mean(guess_histogram)
     )}
  end

  def render(assigns) do
    case assigns[:state] do
      :remaining -> render(HelpleWeb.PageView, "remaining.html", assigns)
      :stats -> render(HelpleWeb.PageView, "stats.html", assigns)
    end
  end

  defp parse_remaining_words_input(session) do
    {
      session["solution_field"],
      Enum.map(0..5, &session[Integer.to_string(&1)])
    }
  end

  defp build_histogram(session) do
    # Extract histogram values from fields
    histogram =
      0..5
      |> Enum.map(&session["h#{Integer.to_string(&1)}"])
      |> Enum.map(fn f ->
        case Integer.parse(f) do
          {v, _} -> v
          :error -> 0
        end
      end)

    # Pad with a 0 out the front.
    # Rust stats functions use indexes as histogram buckets
    [0 | histogram]
  end

  defp calc_mean(histogram) do
    histogram
    |> Helple.RustSolver.external_mean()
    |> Float.round(3)
  end

  defp calc_std_dev(histogram) do
    histogram
    |> Helple.RustSolver.external_std_dev()
    |> Float.round(3)
  end

  defp calc_n(histogram) do
    histogram
    |> Helple.RustSolver.external_count()
  end
end

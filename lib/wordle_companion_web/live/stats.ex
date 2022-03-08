defmodule WordleCompanionWeb.Stats do
  use WordleCompanionWeb, :live_view
  require Logger

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       guess_histogram: ["", "", "", "", "", "", ""],
       calc_time: 0,
       mean: 0,
       std_dev: 0
     )}
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
    render(WordleCompanionWeb.PageView, "stats.html", assigns)
  end
end
